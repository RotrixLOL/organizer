use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process
};
use directories::{BaseDirs, ProjectDirs};

pub fn init_config() -> Result<PathBuf, Box<dyn Error>> {
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

pub fn expand_home_dir(path: &str) -> PathBuf {
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

pub fn move_file(file_path: &Path, dir: &Path) {
    let file_name = file_path.file_name().unwrap();
    let dest_path = dir.join(file_name);
    match fs::rename(&file_path, &dest_path) {
        Ok(_) => println!("moved {} to {}", file_path.display(), dest_path.display()),
        Err(e) => println!("error moving {}: {}", file_path.display(), e),
    }
}
