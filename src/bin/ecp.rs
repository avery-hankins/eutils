use std::env;
use std::process;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Preferences {
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

fn get_config_path() -> std::path::PathBuf {
    let home = env::var("HOME").expect("HOME environment variable not set");
    std::path::Path::new(&home).join(CONFIG_PATH)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return;
    }

    let config_path = get_config_path();
    let preferences = parse_config(&config_path);

    let source_files = &args[1..args.len()-1];
    let destination = &args[args.len() - 1];

    let source_paths = source_files.into_iter().map(|s| split_extension(s));
    dbg!(&source_files);
    dbg!(&source_paths);
    let (end_path, end_name, end_extension) = split_extension(destination);

    for source in source_paths {
        let (start_parent, start_name, start_extension) = source;

        // TODO cleanup fp hell
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
                                                            .map(|f| f.members.iter().any(|ext| end_extension == **ext))
                                                            .unwrap_or(false)
                                                    })
                                                    .map(|(_, cmd)| cmd)
                                                    .collect();

        let command: &Command;
        match matched_commands.len() {
            1 => {command = matched_commands[0]},
            0 => {panic!("No file format matched for: {start_extension}")},
            _ => {panic!("Multiple file formats matched for: {start_extension}")},
        }

        // execute command
        let split_command: Vec<&str> = command.split(" ").collect();
        let mut exe_com = &mut process::Command::new(split_command[0]);
        
        for arg in &split_command[1..] {
            let mapped_arg = match *arg {
                START_FILL => format!("{}{}{}", start_parent, start_name, start_extension),
                END_FILL if end_name.len() == 0 => format!("{}{}{}", end_path, start_name, end_extension),
                END_FILL => format!("{}{}{}", end_path, end_name, end_extension),
                _ => arg.to_string(),
            };

            exe_com = exe_com.arg(mapped_arg);
        }

        dbg!(&exe_com);

        exe_com.status().expect("Command failed");
    }
}

// TODO many of these methods should return results

// TODO better handle unwraps
fn parse_config(config_path: &std::path::Path) -> Preferences {
    match config_path.try_exists().unwrap() {
        false => create_config(config_path),
        _ => ()
    }

    let contents = std::fs::read_to_string(config_path).expect("Failed to read config file");

    serde_json::from_str(&contents).expect("Failed to parse config file")
}

fn create_config(config_path: &std::path::Path) {
    let json_data = r#"{
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
        std::fs::create_dir_all(parent).expect("Failed to create config directory");
    }
    std::fs::write(config_path, json_data).expect("Failed to write config file");
}

fn print_help() {
    // println!("usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file");
    // println!("       cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory");
    
    println!("usage: ecp source_file ... target_file");
}

fn split_extension(file: &str) -> (String, String, FileType) {
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

    if extension.len() == 0 && &stem[0..1] == "." {
        (name, extension, stem) // to deal with how these are extracted
    } else {
        (name, stem, extension)
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
}
