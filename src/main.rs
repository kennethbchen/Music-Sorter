use std::io::Error;

use std::os::windows::process;
use std::process::Output;
use std::{fs, path::PathBuf, str::FromStr};

use std::fs::File;

use config::Config;

use zip::result::ZipError;
use zip::ZipArchive;

use id3::{Tag, TagLike};

struct AppSettings {
    input_path: PathBuf,
    output_path: PathBuf,
    processed_input_path: PathBuf
}

enum SupportedInput {
    DirInput(PathBuf),
    ZipInput(PathBuf),
    Mp3Input(PathBuf)
}

impl SupportedInput {
    fn from(path: PathBuf) -> Option<SupportedInput> {

        if path.is_dir() {
            return Option::Some(SupportedInput::DirInput(path));
        }

        match path.extension() {
            Some(extension_str) => {

                match extension_str.to_str().unwrap() {
                    "zip" => {
                        return Option::Some(SupportedInput::ZipInput(path));
                    },
                    "mp3" => {
                        return Option::Some(SupportedInput::Mp3Input(path));
                    },
                    _ => return None

                }
            }

            None => return None
        }
    }
}

struct SortableTag {
    _title: String,
    album: Option<String>,
    _artist: Option<String>,
    album_artist: Option<String>
}

impl SortableTag {
    fn from(tag: Tag) -> Option<Self>{

        if tag.title().is_none() {
            return None;
        }

        // At least one of these must exist
        if tag.artist().is_none() && tag.album_artist().is_none() {
            return None;
        }

        return Some(SortableTag {
            _title: String::from(tag.title()?),
            album: Some(String::from(tag.album()?)),
            _artist: Some(String::from(tag.artist()?)),
            album_artist: Some(String::from(tag.album_artist()?))
        });

    }

    fn path(&self) -> PathBuf {

        let mut output = PathBuf::new();
        
        let root_folder = self.album_artist.as_ref().unwrap_or_else(|| self.album.as_ref().unwrap());
        output.push(root_folder.clone());

        if let Some(album) = &self.album {
            output.push(album.clone());
        }

        return output;
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

    let processed_input_path = &config.get_string("processed_input_folder").unwrap();
    let processed_input_path: PathBuf = PathBuf::from_str(processed_input_path).unwrap();

    // Validate
    assert!(input_path.try_exists().expect("Could not check if input folder exists."), "Input folder does not exist!");
    assert!(output_path.try_exists().expect("Could not check if output folder exists."), "Output folder does not exist!");
    assert!(processed_input_path.try_exists().expect("Could not check if processed input folder exists."), "Processed input folder does not exist!");

    return AppSettings {
        input_path,
        output_path,
        processed_input_path
    }
}


fn get_destination_path(output_folder_path: &PathBuf, file: &SupportedInput) -> Option<PathBuf> {
    match file {
        SupportedInput::DirInput(path) => todo!(),
        SupportedInput::ZipInput(path) => {
            

            let Ok(zip) = File::open(path) else {return None};

            let Ok(mut zip) = ZipArchive::new(zip) else {return None};
            
            let mut sortable_tag: Option<SortableTag> = None;

            // Find an Mp3 in the zip to get metadata
            for i in 0..zip.len() {
                
                // Check if this file is an mp3
                {
                    let Ok(file) = zip.by_index(i) else {continue};

                    let Some(filepath) = file.enclosed_name() else {continue};

                    let Some(file) = SupportedInput::from(filepath) else {continue};
                    
                    match file {
                        SupportedInput::Mp3Input(_) => (),
                        _ => continue
                    }
                }
                
                // Get song metadata
                let Ok(file) = &mut zip.by_index_seek(i) else {continue};
                
                let Ok(tag) = Tag::read_from2(file) else {continue};
                
                sortable_tag = SortableTag::from(tag);
                
                match sortable_tag {
                    Some(_) => {break},
                    _ => {continue}
                }

            }
            
            let Some(sortable_tag) = sortable_tag else {return None};

            let mut output = output_folder_path.clone();
            output.push(sortable_tag.path());

            return Some(output);
        },
        SupportedInput::Mp3Input(path) => todo!()
    }
}

fn main() {

    let settings: AppSettings = parse_config();

    for entry in fs::read_dir(&settings.input_path).unwrap() {
        
        let Ok(entry) = entry else {continue};

        let entry = SupportedInput::from(entry.path());

        match entry {
            Some(input) => {

                let Some(destination_path) = get_destination_path(&settings.output_path, &input) else {
                    println!("Error getting destination path, skipping");
                    continue;
                };

                match input {
                    SupportedInput::ZipInput(input_path) => {
                        
                        {
                            // Extract every file to destination path
                            let Ok(zip) = File::open(&input_path) else {
                                println!("Couldn't open {:?}. Skipping...", &input_path);
                                continue;
                            };

                            let Ok(mut zip) = ZipArchive::new(zip) else {
                                println!("Couldn't open {:?} as a zip file. Skipping...", &input_path);
                                continue;
                            };
                            
                            
                            let Ok(_) = zip.extract(&destination_path) else {
                                println!("Error extracting {:?}. Skipping...", &input_path);
                                continue;
                            };
                            
                            println!("Extracted {:?} to {:?}", &input_path, &destination_path);

                        }
                        
                        {
                            // Move the input zip to processed input folder
                            let Some(filename) = input_path.file_name() else {
                                println!("Error moving {:?} to {:?}", &input_path, &settings.processed_input_path);
                                continue;
                            };
                            
                            if let Err(er) = fs::rename(&input_path, &settings.processed_input_path.join(filename)) {
                                println!("Error moving {:?} to {:?}", &input_path, &settings.processed_input_path);
                                println!("{}", er);
                                println!();
                                continue;

                            } else {
                                println!("Moved {:?} to {:?}", &input_path, &settings.processed_input_path.join(filename));
                                
                            };
                        }

                        println!();
                        
                        
                    },
                    _ => todo!()
                }

            },
            None => println!("Skipping unsupported input...")
        }

    }


}

