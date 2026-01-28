use std::env;
use std::process;
use std::io::Write;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Preferences {
    warn_dangerous: bool,
    file_formats: Vec<FileFormat>,
}

type Command = String;

// ie .png, .jpg, .mp4, .rs, .exe
type FileType = String;

#[derive(Deserialize, Debug)]
struct FileFormat {
    name: String,
    members: Vec<FileType>,
    transformations: Vec<(String, Command)>,
}

const START_FILL: &str = "{s}";
const END_FILL: &str = "{e}";
const CONFIG_PATH: &str = ".config/eutils/preferences.json";

pub fn get_config_path() -> std::path::PathBuf {
    let home = env::var("HOME").expect("HOME environment should be set");
    std::path::Path::new(&home).join(CONFIG_PATH)
}

// TODO many of these methods should return results

// TODO better handle unwraps
pub fn parse_config(config_path: &std::path::Path) -> Preferences {
    if !config_path.try_exists().unwrap() {
        create_config(config_path);
    }

    let contents = std::fs::read_to_string(config_path).expect("Config path should exist");

    serde_json::from_str(&contents).expect("Config file should be parseable. Syntax error?")
}

fn create_config(config_path: &std::path::Path) {
    let json_data = r#"{
    "warn_dangerous": true,
    "file_formats": [
        {
            "name": "image",
            "members": [".png", ".jpg", ".jpeg", ".webp"],
            "transformations": [
                ["image", "magick {s} {e}"]
            ]
        },
        {
            "name": "video",
            "members": [".mp4", ".mov", ".avi", ".mkv"],
            "transformations": [
                ["video", "ffmpeg -i {s} {e}"]
            ]
        }
    ]
}"#;

    if let Some(parent) = config_path.parent() {
        std::fs::create_dir_all(parent).expect("Should be able to create directory");
    }
    std::fs::write(config_path, json_data).expect("Should be able to create config_file");
}

fn split_extension(file: &str) -> (String, String, FileType) {
    if file == "." || file == ".." {
        return (file.to_string(), "".to_string(), "".to_string());
    }

    let path = std::path::Path::new(file);

    let name = path.parent()
                   .and_then(|s| s.to_str())
                   .unwrap_or_default()
                   .to_string();

    let stem = path.file_stem()
                   .and_then(|s| s.to_str())
                   .unwrap_or_default()
                   .to_string();

    let extension = path.extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext| format!(".{}", ext))
                        .unwrap_or_default();

    if extension.is_empty() && &stem[0..1] == "." {
        (name, extension, stem) // to deal with how these are extracted
    } else {
        (name, stem, extension)
    }
}

/// # Returns
/// Whether the user wants to quit the operation
fn warn_user() -> bool {
    print!("This conversion may lose information in the source files. Are you sure you want to continue? (y/n): ");
    std::io::stdout().flush().unwrap();

    for line in std::io::stdin().lines() {
        let line = line.unwrap();

        if line.trim() == "y" || line.trim() == "Y" {
            return true;
        } else if line.trim() == "n" || line.trim() == "N" {
            return false;
        } else {
            print!("Invalid character. Are you sure you want to continue? (y/n): ");
            std::io::stdout().flush().unwrap();
        }
    }

    panic!("unable to warn user");
}

pub fn execute_on(source_files: &[String], dest: &str, delete_source: bool, preferences: Preferences) {
    if delete_source && preferences.warn_dangerous {
        match warn_user() {
            true => {}, // do nothing
            false => {return}
        }
    }

    let (end_path, end_name, end_extension) = split_extension(dest);
    let is_dest_dir = std::path::Path::new(dest).is_dir();

    for source in source_files {
        let (_start_parent, start_name, start_extension) = split_extension(source);

        // Compute destination file path and target extension
        let (dest_file, target_extension): (String, String) = if is_dest_dir {
            // Destination is an existing directory - keep original extension
            let df = std::path::Path::new(dest)
                .join(format!("{}{}", start_name, start_extension))
                .to_string_lossy()
                .to_string();
            (df, start_extension.clone())
        } else if end_name.is_empty() {
            // No dest name: e.g., "dir1/.png" or ".webp" or "dir1/"
            let ext = if end_extension.is_empty() { start_extension.clone() } else { end_extension.clone() };
            let df = if end_path.is_empty() {
                format!("{}{}", start_name, ext)
            } else {
                std::path::Path::new(&end_path)
                    .join(format!("{}{}", start_name, ext))
                    .to_string_lossy()
                    .to_string()
            };
            (df, ext)
        } else {
            // Full destination path with name
            let df = if end_path.is_empty() {
                format!("{}{}", end_name, end_extension)
            } else {
                std::path::Path::new(&end_path)
                    .join(format!("{}{}", end_name, end_extension))
                    .to_string_lossy()
                    .to_string()
            };
            (df, end_extension.clone())
        };

        // No conversion needed - use filesystem copy
        if start_extension == target_extension {
            std::fs::copy(source, &dest_file).expect("Should be able to copy file");
            if delete_source {
                std::fs::remove_file(source).expect("Should be able to delete source file");
            }
            continue;
        }

        // Conversion needed - look up transformation
        let matched_formats: Vec<&FileFormat> = preferences.file_formats
            .iter()
            .filter(|s| s.members.iter().any(|extension| start_extension == **extension))
            .collect();

        let format: &FileFormat;
        match matched_formats.len() {
            1 => {format = matched_formats[0]},
            0 => {panic!("No file format matched for: {start_extension}")},
            _ => {panic!("Multiple file formats matched for: {start_extension}")},
        }

        let matched_commands: Vec<&Command> = format.transformations
            .iter()
            .filter(|(target_name, _)| {
                preferences.file_formats
                    .iter()
                    .find(|f| &f.name == target_name)
                    .map(|f| f.members.iter().any(|ext| target_extension == **ext))
                    .unwrap_or(false)
            })
            .map(|(_, cmd)| cmd)
            .collect();

        let command: &Command;
        match matched_commands.len() {
            1 => {command = matched_commands[0]},
            0 => {panic!("No file format matched for: {target_extension}")},
            _ => {panic!("Multiple file formats matched for: {target_extension}")},
        }

        // execute command
        let split_command: Vec<&str> = command.split(" ").collect();
        let mut exe_com = &mut process::Command::new(split_command[0]);

        for arg in &split_command[1..] {
            let mapped_arg = match *arg {
                START_FILL => source.to_string(),
                END_FILL => dest_file.clone(),
                _ => arg.to_string(),
            };

            exe_com = exe_com.arg(mapped_arg);
        }

        let status = exe_com.status().expect("Should be able to execute specified command");
        if status.success() && delete_source {
            std::fs::remove_file(source).expect("Should be able to delete source file");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        assert_eq!(split_extension("foo/bar.txt"), ("foo".to_string(), "bar".to_string(), ".txt".to_string()));

        assert_eq!(split_extension("a/b/bar.txt"), ("a/b".to_string(), "bar".to_string(), ".txt".to_string()));

        assert_eq!(split_extension(".txt"), ("".to_string(), "".to_string(), ".txt".to_string()));

        assert_eq!(split_extension(".log.txt"), ("".to_string(), ".log".to_string(), ".txt".to_string()));
    }

    #[test]
    fn test_split_directory() {
        assert_eq!(split_extension("a/b/"), ("a".to_string(), "b".to_string(), "".to_string()));
    }
}
