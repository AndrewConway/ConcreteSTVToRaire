use std::fs::File;
use std::path::PathBuf;
use anyhow::anyhow;

use clap::{Parser};
use serde_json::json;
use raire::audit_type::{Audit, BallotComparisonMACRO, BallotComparisonOneOnDilutedMargin, BallotPollingBRAVO, BallotPollingOneOnDilutedMarginSquared};
use raire::irv::{Votes};
use raire::RaireProblem;
use raire::timeout::TimeOut;
use stv::election_data::ElectionData;

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// This reads the CSV files in the https://github.com/michelleblom/audit-irv-cp/tree/raire-branch repo and converts them to
/// the JSON unput for raire-rs
struct CliOptions {
    /// The .stv file from ConcreteSTV
    input_raire_file : PathBuf,
    /// The raire-rs output file. Default is the input file name, with path and extension if present removed and `.json` added.
    output_json_file : Option<PathBuf>,
    /// set if you want ballot polling (default ballot comparison)
    #[arg(long)]
    ballot_polling : bool,
    /// the total number of ballots (if different from the number of votes in the file)
    #[arg(long)]
    total_ballots : Option<usize>,
    /// the desired confidence level (for MACRO or BRAVO). If not specified, then a 1/margin (or 1/margin squared) computation will be done.
    #[arg(long)]
    confidence : Option<f64>,
    /// the error_inflation_factor (for MACRO).
    #[arg(long)]
    error_inflation_factor : Option<f64>,
}

fn main() -> anyhow::Result<()> {
    let args = CliOptions::parse();
    let input : ElectionData = serde_json::from_reader(File::open(&args.input_raire_file)?)?;
    let output = {
        let num_ballots : usize = input.num_votes();
        println!("{num_ballots} ballots");
        let total_auditable_ballots = raire::irv::BallotPaperCount(args.total_ballots.unwrap_or(num_ballots));
        let audit : Audit = match (args.ballot_polling,args.confidence) {
            (false,None) => Audit::OneOnMargin(BallotComparisonOneOnDilutedMargin{ total_auditable_ballots }),
            (true,None) => Audit::OneOnMarginSq(BallotPollingOneOnDilutedMarginSquared{ total_auditable_ballots }),
            (false,Some(confidence)) => Audit::MACRO(BallotComparisonMACRO{total_auditable_ballots,confidence,error_inflation_factor:args.error_inflation_factor.unwrap_or(1.0)}),
            (true,Some(confidence)) => Audit::BRAVO(BallotPollingBRAVO{total_auditable_ballots,confidence}),
        };
        let arena = typed_arena::Arena::<stv::ballot_metadata::CandidateIndex>::new();
        let votes = input.resolve_atl(&arena,None);
        let votes : Vec<raire::irv::Vote> = votes.into_iter().map(|v|raire::irv::Vote{ n: raire::irv::BallotPaperCount(v.n.0), prefs: v.prefs.iter().map(|c|raire::irv::CandidateIndex(c.0 as u32)).collect() }).collect();
        let votes = Votes::new(votes,input.metadata.candidates.len());
        let candidates : Vec<String> = input.metadata.candidates.iter().map(|c|c.name.clone()).collect();
        let result = votes.run_election(&mut TimeOut::never())?;
        if result.possible_winners.len()!=1 { return Err(anyhow!{"Multiple winners - cannot audit"}); }
        RaireProblem{
            metadata: json!({
                "candidates" : candidates
            }),
            num_candidates: input.metadata.candidates.len(),
            votes : votes.votes,
            winner: Some(result.possible_winners[0]),
            audit,
            trim_algorithm: None,
            difficulty_estimate: None,
            time_limit_seconds: None,
        }
    };
    let output_file : PathBuf = args.output_json_file.unwrap_or_else(||{
        let mut stem = args.input_raire_file.file_stem().map(|s|PathBuf::from(s)).unwrap_or_else(||PathBuf::from("output"));
        stem.as_mut_os_string().push(".json");
        stem
    });
    serde_json::to_writer(File::create(&output_file)?,&output)?;
    Ok(())
}