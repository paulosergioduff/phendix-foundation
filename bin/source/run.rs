use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Verifica se o nome do projeto foi fornecido como argumento
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Erro: O nome do projeto é necessário.");
        std::process::exit(1);
    }
    let project_name = &args[1];

    // Caminhos para as pastas de prompt e documentação
    let prompt_folder = format!("projects/{}/auto/prompt/GPT-3.5_v1/", project_name);
    let doc_folder = format!("projects/{}/doc/", project_name);

    // Verifica a existência da pasta de documentação
    if !Path::new(&doc_folder).exists() {
        eprintln!("Faltando documentação na pasta {} na pasta doc/", prompt_folder);
        std::process::exit(1);
    }

    // Obtém a lista de arquivos na pasta de prompt
    let prompt_files = fs::read_dir(&prompt_folder)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<String>>();

    // Verifica cada arquivo de prompt
    for file_name in prompt_files {
        let prompt_path = format!("{}{}", &prompt_folder, &file_name);
        let doc_path = format!("{}{}", &doc_folder, &file_name);

        // Verifica se o arquivo de prompt tem um equivalente na pasta de documentação
        if !Path::new(&doc_path).exists() {
            eprintln!("Faltando documentação para {} na pasta {}", &file_name, &doc_folder);
            std::process::exit(1);
        }

        // Verifica o tamanho do arquivo de documentação em relação ao arquivo de prompt
        let prompt_metadata = fs::metadata(&prompt_path).unwrap();
        let doc_metadata = fs::metadata(&doc_path).unwrap();
        if doc_metadata.len() < prompt_metadata.len() {
            eprintln!(
                "Erro: O arquivo de documentação {} é menor que seu correspondente em {}",
                &doc_path, &prompt_path
            );
            std::process::exit(1);
        }
    }

    // Se todas as verificações passarem, exibe uma mensagem de sucesso
    println!("Todas as regras de negócio foram executadas com sucesso!");
}
