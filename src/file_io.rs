use crate::models::Ym2151Log;
use std::error::Error;

/// Load a JSON file containing YM2151 log data
pub fn load_file(path: &str) -> Result<Ym2151Log, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?;
    let log = serde_json::from_str(&content)?;
    Ok(log)
}

/// Save YM2151 log data to a JSON file
pub fn save_file(path: &str, log: &Ym2151Log) -> Result<(), Box<dyn Error>> {
    let content = serde_json::to_string_pretty(log)?;
    std::fs::write(path, content)?;
    Ok(())
}
