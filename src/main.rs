use directories::{BaseDirs, ProjectDirs};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process,
};
use toml::Value;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Staring loop, this app will run forever; if you want to stop it, just do this command:\nctrl + c");
    eprintln!("You can put this in systemd or another process supervisor if you want");

    // parse config
    let config_path = init_config()?;
    let config_contents = fs::read_to_string(&config_path).unwrap();
    let config = config_contents.parse::<Value>().unwrap();
    let directories = config["directories"].as_table().unwrap();

    let downloads_dir = expand_home_dir(directories["downloads"].as_str().unwrap());
    let images_dir = expand_home_dir(directories["images"].as_str().unwrap());
    let documents_dir = expand_home_dir(directories["documents"].as_str().unwrap());
    let music_dir = expand_home_dir(directories["music"].as_str().unwrap());
    let videos_dir = expand_home_dir(directories["videos"].as_str().unwrap());

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

fn init_config() -> Result<PathBuf, Box<dyn Error>> {
    let project_dirs = ProjectDirs::from("", "", "organizer").unwrap();
    let config_dir = project_dirs.config_dir();

    if !config_dir.exists() {
        fs::create_dir_all(&config_dir)?;
    }

    let config_path = config_dir.join("config.toml");

    // create config
    if !config_path.exists() {
        let default_config = r#"[directories]
downloads = "~/Downloads"
images = "~/Pictures"
documents = "~/Documents"
music = "~/Music"
videos = "~/Movies"
"#;
        fs::write(&config_path, default_config).unwrap();
        println!("Created default config file at {}", config_path.display());
        println!("check the config and run again");

        // stop the process with exit code 0 (good exit)
        process::exit(0);
    }

    Ok(config_path)
}

fn expand_home_dir(path: &str) -> PathBuf {
    let mut expanded = PathBuf::new();
    if path.starts_with("~/") {
        if let Some(home_dir) = BaseDirs::new().map(|dirs| dirs.home_dir().to_path_buf()) {
            expanded.push(home_dir);
            expanded.push(&path[2..]);
        } else {
            eprintln!("Unable to expand home directory in path: {}", path);
            process::exit(1);
        }
    } else {
        expanded.push(path);
    }
    expanded
}

fn move_file(file_path: &Path, dir: &Path) {
    let file_name = file_path.file_name().unwrap();
    let dest_path = dir.join(file_name);
    match fs::rename(&file_path, &dest_path) {
        Ok(_) => println!("moved {} to {}", file_path.display(), dest_path.display()),
        Err(e) => println!("error moving {}: {}", file_path.display(), e),
    }
}
