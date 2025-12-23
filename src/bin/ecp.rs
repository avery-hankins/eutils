use std::env;

struct Preferences {
    file_formats: Vec<FileFormat>,
}

type Command = String;
type FileType = String;

struct FileFormat {
    members: Vec<FileType>,
    transformations: (&'static FileFormat, Command),
}

fn main() {
    println!("Hello, world!");

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
}

fn parse_config() {
    todo!();
}

fn print_help() {
    todo!();
}

fn get_extension(file: &str) -> FileType {
    todo!();
}
