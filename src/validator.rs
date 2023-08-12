use crate::files::similars_from_yaml::load as load_similars;
// Validator function
pub fn validate() -> Result<(), Box<dyn std::error::Error>> {
    load_similars()?;
    Ok(())
}
