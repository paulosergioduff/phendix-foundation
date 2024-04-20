// help.rs

pub fn print_help() {
    let help_text = get_help_text();
    println!("{}", help_text);
}

fn get_help_text() -> &'static str {
    "
    Usage: Phendix Foundation CLI

    Actions:
      run       Run a command. 
        Sintaxe: $phen run project_name extension model_language
        Example: $phen run example_project js gpt3  
      start     Start a project
        Sintaxe: $phen NewApp

    Options:
    --help    Show help message
    -v               Show version
    --version        Show version (the same command)
    "
}
