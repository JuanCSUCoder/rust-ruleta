use std::{fs, thread, time::Duration};

use log::{error, info, warn};
use rand::{RngExt, SeedableRng, rngs::{ChaCha20Rng, ThreadRng}, seq::IndexedRandom};
use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
enum ProgramError {
    #[error("Error fetching OS entropy: {0}")]
    EntropyFetchingError(#[from] std::io::Error),

    #[error("Available entropy is too low for trusted operations.")]
    LowEntropyError,

    #[error("Error parsing victims.json file, please check if the file is valid JSON and the structure is correct: {0}")]
    VictimsParsingError(#[from] serde_json::Error),

    #[error("Error: victims list is empty.")]
    EmptyVictimsListError,

    #[error("Error: options list is empty.")]
    EmptyOptionsListError,
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

    thread::sleep(Duration::from_secs(5));

    let mut rng = ChaCha20Rng::from_rng(&mut ThreadRng::default());

    let mut random_loops = rng.random_range(30..=50);

    let mut random_person = victims.persons.choose(&mut rng).ok_or(ProgramError::EmptyVictimsListError)?;

    for i in 0..random_loops {
        random_person = victims.persons.choose(&mut rng).ok_or(ProgramError::EmptyVictimsListError)?;

        warn!("{}", i + 1, random_person);
        thread::sleep(Duration::from_millis(((3000*random_loops)/random_loops)/(random_loops-(i))));
    }

    info!("Selected victim: {}", random_person);

    thread::sleep(Duration::from_secs(5));

    let mut random_option = victims.options.choose(&mut rng).ok_or(ProgramError::EmptyOptionsListError)?;

    for i in 0..random_loops {
        random_option = victims.options.choose(&mut rng).ok_or(ProgramError::EmptyOptionsListError)?;

        warn!("{}", i + 1, random_option);
        thread::sleep(Duration::from_millis(((3000*random_loops)/random_loops)/(random_loops-(i))));
    }

    info!("Selected option: {}", random_option);

    Ok(())
}
