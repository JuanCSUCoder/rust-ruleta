use std::fs;

use log::{error, info};
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
enum ProgramError {
    #[error("Error fetching OS entropy: {0}")]
    EntropyFetchingError(#[from] std::io::Error),

    #[error("Available entropy is too low for trusted operations.")]
    LowEntropyError,

    #[error("Error parsing victims.json file, please check if the file is valid JSON and the structure is correct: {0}")]
    VictimsParsingError(#[from] serde_json::Error),
}

#[derive(Debug, Deserialize)]
struct Victims {
    persons: Vec<String>,
    options: Vec<String>,
}

fn main() -> Result<(), ProgramError> {
    tracing_subscriber::fmt::init();

    let victims: Victims = serde_json::from_str(&fs::read_to_string("victims.json")?)?;

    info!("Loaded victims =====================");
    victims.persons.iter().for_each(|person| {
        info!("Victim: {}", person);
    });

    info!("Loaded options =====================");
    victims.options.iter().for_each(|option| {
        info!("Option: {}", option);
    });

    // Read the current available entropy in bits
    let entropy = fs::read_to_string("/proc/sys/kernel/random/entropy_avail")?;
    
    // Read the total pool size (usually 256 or 4096 depending on kernel version)
    let pool_size = fs::read_to_string("/proc/sys/kernel/random/poolsize")?;

    info!("Available Entropy: {} / {} bits", entropy.trim(), pool_size.trim());

    let entropy_value = match entropy.trim().parse::<u32>() {
        Ok(val) => val,
        Err(_) => 0,
    };

    if entropy_value < 256 {
        error!("Error: Available entropy is low. Aborting calculation!");
        return Err(ProgramError::LowEntropyError);
    }

    info!("Entropy level is safe, proceeding with calculations.");

    Ok(())
}
