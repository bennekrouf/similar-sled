use crate::files::similars_from_yaml::load;

// Validator function
pub fn validate() -> Result<(), Box<dyn std::error::Error>> {
    load()?;
    Ok(())
}
