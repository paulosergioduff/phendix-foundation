use std::env;
use std::io;

mod help;
mod run;
mod start;

fn main() -> io::Result<()> {
    // Check if an action was provided as argument
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Error: An action (run or start) must be provided as argument.");
        std::process::exit(1);
    }

    // Action provided as argument
    let action = &args[1];

    match action.as_str() {
        "run" => {
            // Call the run.rs function with the provided arguments
            run_command(&args)?;
        }
        "start" => {
            // Call the existing functionality of phen.rs
            start_command(&args)?;
        }

        "--help" => {
            // Call the existing functionality of phen.rs
            help::print_help();
        }

        "-h" => {
            // Call the existing functionality of phen.rs
            help::print_help();
        }

        "-v" => {
            // Call the existing functionality of phen.rs
            version();
        }

        "--version" => {
            // Call the existing functionality of phen.rs
            version();
        }
        _ => {
            eprintln!("Error: Unknown action. Valid actions are 'run' and 'start'.");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn version(){
    println!("Phendix Foundation CLI - Version Alpha 0.0.1");
}

fn start_command(args: &[String]) -> io::Result<()> {
    // Check if a project name was provided as argument
    if args.len() < 2 {
        eprintln!("Error: A project name must be provided as argument.");
        std::process::exit(1);
    }

    // Project name provided as argument
    let project_name = &args[2];

    // Create the folder and file structure of the project
    start::create_project_structure(project_name)?;

    Ok(())
}

fn run_command(args: &[String]) -> io::Result<()> {
    // Call the run.rs function with the provided arguments
    run::run_command(args)
}
