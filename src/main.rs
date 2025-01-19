use std::{collections::HashMap, path::Path};

use config::Config;

fn main() {

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
    let input_path: &Path = Path::new(input_path);

    let output_path = &config.get_string("output_folder").unwrap();
    let output_path: &Path = Path::new(output_path);

    assert!(input_path.try_exists().expect("Could not check if input folder exists."), "Input folder does not exist!");
    assert!(output_path.try_exists().expect("Could not check if output folder exists."), "Output folder does not exist!");

    println!("{:?}", input_path);
    println!("{:?}", output_path);
    
    
    
}

