use std::env;

struct Preferences {
    file_formats: Vec<FileFormat>,
}

type Command = String;

// ie .png, .jpg, .mp4, .rs, .exe
type FileType = String;

struct FileFormat {
    members: Vec<FileType>,
    transformations: Vec<(&'static FileFormat, &'static Command)>,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return;
    }

    // parse config (.yaml)
    let preferences = parse_config();
    // execute command specified in config
    let file1 = &args[1];
    let file2 = &args[2];
    let start_extension = get_extension(file1);
    let end_extension = get_extension(file2);

    let matched_formats: Vec<FileFormat> = preferences.file_formats
                                                      .into_iter()
                                                      .filter(|s| s.members.iter().any(|extension| start_extension == **extension))
                                                      .collect();

    let format: &FileFormat;
    match matched_formats.len() {
        1 => {format = &matched_formats[0]},
        0 => {panic!("No file format matched for: {start_extension}")},
        _ => {panic!("Multiple file formats matched for: {start_extension}")},
    }

    let matched_commands: Vec<&Command> = format.transformations
                                      .iter()
                                      .filter(|s| s.0.members.iter().any(|exstension| end_extension == **exstension))
                                      .map(|s| s.1)
                                      .collect();

    let command: &Command;
    match matched_commands.len() {
        1 => {command = &matched_commands[0]},
        0 => {panic!("No file format matched for: {start_extension}")},
        _ => {panic!("Multiple file formats matched for: {start_extension}")},
    }

    // fill in command template
}

fn parse_config() -> Preferences {
    todo!();
}

fn print_help() {
    todo!();
}

fn get_extension(file: &str) -> FileType {
    todo!();
}
