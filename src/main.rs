use std::{collections::HashMap, fs, path::{Path, PathBuf}, str::FromStr};

use config::Config;

struct AppSettings {
    input_path: PathBuf,
    output_path: PathBuf
}

fn parse_config() -> AppSettings {

    // Parse config file
    let config = Config::builder()
        .add_source(config::File::with_name("config.toml"))
        .build();

    match config {
        Ok(_) => (),
        Err(_) => panic!("config.toml not found")
    }

    let config: Config = config.unwrap();

    // Get settings from config file
    let input_path = &config.get_string("input_folder").unwrap();
    let input_path: PathBuf = PathBuf::from_str(input_path).unwrap();

    let output_path = &config.get_string("output_folder").unwrap();
    let output_path: PathBuf = PathBuf::from_str(output_path).unwrap();

    // Validate
    assert!(input_path.try_exists().expect("Could not check if input folder exists."), "Input folder does not exist!");
    assert!(output_path.try_exists().expect("Could not check if output folder exists."), "Output folder does not exist!");

    return AppSettings {
        input_path,
        output_path
    }
}
fn main() {
    
    let settings: AppSettings = parse_config();

    for entry in fs::read_dir(&settings.input_path).unwrap() {
        let entry = entry.unwrap();

        println!("{:?}", entry.path());

    }


}

