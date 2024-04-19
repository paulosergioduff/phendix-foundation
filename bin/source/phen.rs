use std::env;
use std::io;

mod run;
mod start;

fn main() -> io::Result<()> {
    // Verifica se foi fornecido um nome de ação como argumento
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Erro: É necessário fornecer uma ação (run ou start) como argumento.");
        std::process::exit(1);
    }

    // Ação fornecida como argumento
    let action = &args[1];

    match action.as_str() {
        "run" => {
            // Chama a função de run.rs com os argumentos fornecidos
            run_command(&args)?;
        }
        "start" => {
            // Chama a funcionalidade existente do phen.rs
            start_command(&args)?;
        }

        "--help" => {
            // Chama a funcionalidade existente do phen.rs
            help();
        }

        "-v" => {
            // Chama a funcionalidade existente do phen.rs
            version();
        }
        _ => {
            eprintln!("Erro: Ação desconhecida. As ações válidas são 'run' e 'start'.");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn help(){
    println!("Isso será um arquivo de ajuda!");
}

fn version(){
    println!("Phendix Foundation CLI - Version Alpha 0.0.1");
}

fn start_command(args: &[String]) -> io::Result<()> {
    // Verifica se foi fornecido um nome de projeto como argumento
    if args.len() < 2 {
        eprintln!("Erro: É necessário fornecer um nome de projeto como argumento.");
        std::process::exit(1);
    }

    // Nome do projeto fornecido como argumento
    let nome_do_projeto = &args[2];

    // Cria a estrutura de pastas e arquivos do projeto
    start::create_project_structure(nome_do_projeto)?;


    Ok(())
}

fn run_command(args: &[String]) -> io::Result<()> {
    // Chama a função de run.rs com os argumentos fornecidos
    run::run_command(args)
}
