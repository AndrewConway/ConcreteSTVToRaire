use std::fs::File;
use std::path::PathBuf;
use anyhow::anyhow;
use raire::audit_type::{Audit, BallotComparisonOneOnDilutedMargin};
use raire::irv::Votes;
use raire::RaireProblem;
use raire::timeout::TimeOut;
use stv::election_data::ElectionData;
use serde_json::json;
use stv::ballot_metadata::NumberOfCandidates;
use stv::datasource_description::ElectionDataSource;
use stv::parse_util::FileFinder;

pub fn convert(input:&ElectionData,audit : Audit) -> anyhow::Result<RaireProblem> {
    let arena = typed_arena::Arena::<stv::ballot_metadata::CandidateIndex>::new();
    let votes = input.resolve_atl(&arena,None);
    let votes : Vec<raire::irv::Vote> = votes.into_iter().map(|v|raire::irv::Vote{ n: raire::irv::BallotPaperCount(v.n.0), prefs: v.prefs.iter().map(|c|raire::irv::CandidateIndex(c.0 as u32)).collect() }).collect();
    let votes = Votes::new(votes,input.metadata.candidates.len())?;
    let candidates : Vec<String> = input.metadata.candidates.iter().map(|c|c.name.clone()).collect();
    let result = votes.run_election(&mut TimeOut::never())?;
    if result.possible_winners.len()!=1 { return Err(anyhow!{"Multiple winners - cannot audit"}); }
    let contest_name = input.metadata.name.human_readable_name();
    Ok(RaireProblem{
        metadata: json!({
                "candidates" : candidates,
                "contest" : contest_name
            }),
        num_candidates: input.metadata.candidates.len(),
        votes : votes.votes,
        winner: Some(result.possible_winners[0]),
        audit,
        trim_algorithm: None,
        difficulty_estimate: None,
        time_limit_seconds: None,
    })
}

/// Convert everything in the ElectionDataSource. If only_one is true, the only convert iff the number of vacancies is exactly one.
pub fn convert_bulk(source:&impl ElectionDataSource,only_one:bool) -> anyhow::Result<()> {
    let finder = FileFinder::find_ec_data_repository();
    let path = PathBuf::from(source.name().as_ref());
    for year in source.years() {
        let path = path.join(&year);
        println!("\nCreating {:?}\n",path);
        std::fs::create_dir_all(&path)?;
        let loader = source.get_loader_for_year(&year,&finder)?;
        for electorate in loader.all_electorates() {
            println!("{}",electorate);
            let path = path.join(format!("{}.json",electorate.replace('/',"_")));
            let data = loader.read_raw_data_best_quality(&electorate)?;
            if data.metadata.vacancies==Some(NumberOfCandidates(1)) || !only_one {
                let audit = Audit::OneOnMargin(BallotComparisonOneOnDilutedMargin{total_auditable_ballots:raire::irv::BallotPaperCount(data.num_votes())});
                let output = convert(&data,audit)?;
                serde_json::to_writer(File::create(&path)?,&output)?;
            }
        }
    }
    Ok(())
}
