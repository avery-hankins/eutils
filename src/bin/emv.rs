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

    execute_on(source_files, destination, true, preferences);
}

fn print_help() {
    // usage: mv [-f | -i | -n] [-hv] source target
    //        mv [-f | -i | -n] [-v] source ... directory
    
    println!("usage: emv source ... target");
}

