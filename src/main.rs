use std::fs;

use log::{error, info};

#[derive(thiserror::Error, Debug)]
enum ProgramError {
    #[error("Error fetching OS entropy: {0}")]
    EntropyFetchingError(#[from] std::io::Error),

    #[error("Available entropy is too low for trusted operations.")]
    LowEntropyError,
}

fn main() -> Result<(), ProgramError> {
    tracing_subscriber::fmt::init();

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
