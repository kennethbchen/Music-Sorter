use std::io::Error;

use std::path::Path;
use std::{fs, path::PathBuf, str::FromStr};

use std::fs::File;

use config::Config;

use zip::ZipArchive;

use id3::{Tag, TagLike};

struct AppSettings {
    input_path: PathBuf,
    output_path: PathBuf
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
    title: String,
    album: Option<String>,
    artist: Option<String>,
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
            title: String::from(tag.title()?),
            album: Some(String::from(tag.album()?)),
            artist: Some(String::from(tag.artist()?)),
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

    // Validate
    assert!(input_path.try_exists().expect("Could not check if input folder exists."), "Input folder does not exist!");
    assert!(output_path.try_exists().expect("Could not check if output folder exists."), "Output folder does not exist!");

    return AppSettings {
        input_path,
        output_path
    }
}

fn get_destination_path(output_folder_path: &PathBuf, file: SupportedInput) -> Result<PathBuf, Error> {
    match file {
        SupportedInput::DirInput(path) => todo!(),
        SupportedInput::ZipInput(path) => {
            

            let zip = File::open(path)?;

            let mut zip = ZipArchive::new(zip)?;
            
            let mut sortable_tag: Option<SortableTag> = None;

            // Find an Mp3 in the zip to get metadata
            for i in 0..zip.len() {
                
                
                
                // Check if this file is an mp3
                {
                    let file = zip.by_index(i)?;

                    let filepath = file.enclosed_name();

                    match &filepath {
                        Option::Some(_) => (),
                        Option::None => {continue;}
                    }

                    let filepath = filepath.unwrap();

                    let file = SupportedInput::from(filepath);

                    if file.is_none() {
                        continue;
                    }

                    let file = file.unwrap();

                    match file {
                        SupportedInput::Mp3Input(_) => (),
                        _ => continue
                    }
                }
                
                // Get song metadata
                let file = &mut zip.by_index_seek(i)?;
                
                let tag = Tag::read_from2(file).unwrap();
                
                sortable_tag = SortableTag::from(tag);
                
                if sortable_tag.is_none() {
                    continue;
                }
                
                break;
            }
            
            return Ok(sortable_tag.unwrap().path());
        },
        SupportedInput::Mp3Input(path) => todo!()
    }
}

fn main() {
    
    let settings: AppSettings = parse_config();

    for entry in fs::read_dir(&settings.input_path).unwrap() {
        let entry = entry.unwrap();

        let entry = SupportedInput::from(entry.path());

        match entry {
            Some(input) => {

                let path = get_destination_path(&settings.output_path, input);

                match path {
                    Result::Ok(_) => (),
                    Result::Err(_) => {
                        println!("Error getting path, skipping");
                        continue;
                    }
                }

                let path = path.unwrap();
                
                println!("{:?}", path);

            },
            None => println!("Skipping unsupported input...")
        }

    }


}

