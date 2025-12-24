use std::env;
use eutils::*;

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

    execute_on(source_files, destination, false, preferences);
}

fn print_help() {
    // println!("usage: cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file target_file");
    // println!("       cp [-R [-H | -L | -P]] [-fi | -n] [-aclpSsvXx] source_file ... target_directory");
    
    println!("usage: ecp source_file ... target_file");
}

