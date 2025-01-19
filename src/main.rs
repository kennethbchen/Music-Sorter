use std::{collections::HashMap, fs, path::{Path, PathBuf}, str::FromStr};

use config::Config;

struct AppSettings {
    input_path: PathBuf,
    output_path: PathBuf
}

enum SupportedInput {
    Dir(PathBuf),
    Zip(PathBuf),
    Mp3(PathBuf)
}


impl SupportedInput {
    fn from(path: PathBuf) -> Option<SupportedInput> {

        if path.is_dir() {
            return Option::Some(SupportedInput::Dir(path));
        }

        match path.extension() {
            Some(extension_str) => {

                match extension_str.to_str().unwrap() {
                    "zip" => {
                        return Option::Some(SupportedInput::Zip(path));
                    },
                    "mp3" => {
                        return Option::Some(SupportedInput::Mp3(path));
                    },
                    _ => return None

                }
            }
            None => return None
        }
    }
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

fn get_destination_path(output_path: &PathBuf, input: SupportedInput) {
    match input {
        SupportedInput::Dir(path) => println!("Dir"),
        SupportedInput::Zip(path) => println!("Zip"),
        SupportedInput::Mp3(path) => println!("Mp3")
    }
}

fn main() {
    
    let settings: AppSettings = parse_config();

    for entry in fs::read_dir(&settings.input_path).unwrap() {
        let entry = entry.unwrap();

        let entry = SupportedInput::from(entry.path());

        match entry {
            Some(input) => {
                get_destination_path(&settings.output_path, input);
            },
            None => println!("Skipping unsupported input...")
        }

    }


}

