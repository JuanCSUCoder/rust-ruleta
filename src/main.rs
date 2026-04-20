use std::fs;

use log::info;

#[derive(thiserror::Error, Debug)]
enum ProgramError {
    #[error("Error fetching OS entropy: {0}")]
    EntropyFetchingError(#[from] std::io::Error),
}

fn main() -> Result<(), ProgramError> {
    tracing_subscriber::fmt::init();

    // Read the current available entropy in bits
    let entropy = fs::read_to_string("/proc/sys/kernel/random/entropy_avail")?;
    
    // Read the total pool size (usually 256 or 4096 depending on kernel version)
    let pool_size = fs::read_to_string("/proc/sys/kernel/random/poolsize")?;

    info!("Available Entropy: {} / {} bits", entropy.trim(), pool_size.trim());

    Ok(())
}
