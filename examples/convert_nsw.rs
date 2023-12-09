
use concrete_stv_to_raire::convert_bulk;

fn main() -> anyhow::Result<()> {
    // convert_bulk(&federal::parse::FederalDataSource{})?;
    convert_bulk(&nsw::parse_lge::NSWLGEDataSource{},true)?;
    //convert_bulk(&nsw::parse_lc::NSWLCDataSource{})?;
    Ok(())
}