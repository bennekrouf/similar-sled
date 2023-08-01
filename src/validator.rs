use crate::files::similars_from_yaml::load as load_similars;
use crate::domain::hadith::mousned_from_yaml::load as load_mousned;
// Validator function
pub fn validate() -> Result<(), Box<dyn std::error::Error>> {
    load_similars()?;
    load_mousned()?;
    Ok(())
}
