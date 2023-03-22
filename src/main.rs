use std::{
    error::Error,
    fs,
};
use toml::Value;
use walkdir::WalkDir;

use organizer_lib as lib;

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Staring loop, this app will run forever; if you want to stop it, just do this command:\nctrl + c");
    eprintln!("You can put this in systemd or another process supervisor if you want");

    // parse config
    let config_path = lib::init_config()?;
    let config_contents = fs::read_to_string(&config_path).unwrap();
    let config = config_contents.parse::<Value>().unwrap();
    let directories = config["directories"].as_table().unwrap();

    let downloads_dir = lib::expand_home_dir(directories["downloads"].as_str().unwrap());
    let images_dir = lib::expand_home_dir(directories["images"].as_str().unwrap());
    let documents_dir = lib::expand_home_dir(directories["documents"].as_str().unwrap());
    let music_dir = lib::expand_home_dir(directories["music"].as_str().unwrap());
    let videos_dir = lib::expand_home_dir(directories["videos"].as_str().unwrap());

    loop {
        for entry in WalkDir::new(&downloads_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_str = ext.to_str().unwrap_or("").to_lowercase();
                    // image
                    if ext_str == "jpg"
                        || ext_str == "jpeg"
                        || ext_str == "png"
                        || ext_str == "webp"
                        || ext_str == "gif"
                    {
                        move_file(path, &images_dir);
                    // document
                    } else if ext_str == "pdf" || ext_str == "docx" || ext_str == "txt" {
                        move_file(path, &documents_dir);
                    // audio
                    } else if ext_str == "mp3" || ext_str == "wav" || ext_str == "ogg" {
                        move_file(path, &music_dir);
                    // video
                    } else if ext_str == "mp4"
                        || ext_str == "avi"
                        || ext_str == "mkv"
                        || ext_str == "mov"
                    {
                        move_file(path, &videos_dir);
                    }
                }
            }
        }
    }
}