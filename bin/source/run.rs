use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn run_command(args: &[String]) -> io::Result<()> {
    // Verifica se foram fornecidos todos os argumentos necessários para a ação 'run'
    if args.len() < 5 {
        eprintln!("\x1b[31mErro: O nome do projeto, modelo de linguagem e a extensão do arquivo são necessários para a ação 'run'.\x1b[0m");
        std::process::exit(1);
    }

    let project_name = &args[2];
    let extension = &args[3];
    let model_language = &args[4];

    // Caminhos para as pastas de prompt e documentação
    let prompt_folder = format!("projects/{}/auto/prompt/{}/", project_name, model_language);
    let doc_folder = format!("projects/{}/doc/", project_name);

    // Verifica a existência da pasta de documentação
    if !Path::new(&doc_folder).exists() {
        eprintln!("\x1b[31mFaltando documentação na pasta {} na pasta doc/\x1b[0m", prompt_folder);
        std::process::exit(1);
    }

    // Verifica se há arquivos na pasta de prompt com a extensão fornecida
    let prompt_files: Vec<PathBuf> = match fs::read_dir(&prompt_folder) {
        Ok(files) => files
            .filter_map(|entry| {
                if let Ok(entry) = entry {
                    if let Some(file_name) = entry.file_name().to_str() {
                        if file_name.ends_with(&format!(".{}", extension)) {
                            return Some(entry.path());
                        }
                    }
                }
                None
            })
            .collect(),
        Err(_) => {
            eprintln!("\x1b[31mErro ao acessar a pasta de código.\x1b[0m");
            std::process::exit(1);
        }
    };

    // Verifica se há arquivos na pasta de código com a extensão especificada
    if prompt_files.is_empty() {
        eprintln!("\x1b[31mNão foram encontrados arquivos com a extensão '{}' na pasta de código.\x1b[0m", extension);
        std::process::exit(1);
    }

    // Verifica se há arquivos correspondentes na pasta de documentação
    let mut missing_docs = Vec::new();
    let mut oversized_files = Vec::new();
    for prompt_file in prompt_files {
        let prompt_file_name = prompt_file.file_name().unwrap().to_string_lossy().to_string();
        // Caminho para o arquivo de documentação correspondente
        let doc_file_name = format!("{}.md", &prompt_file_name);
        let doc_path = Path::new(&doc_folder).join(&doc_file_name);

        // Verifica se o arquivo de documentação existe
        if !doc_path.exists() {
            missing_docs.push(prompt_file_name);
        } else {
            // Verifica o tamanho do arquivo de documentação em relação ao arquivo de prompt
            let prompt_metadata = fs::metadata(&prompt_file).unwrap();
            let doc_metadata = fs::metadata(&doc_path).unwrap();
            if doc_metadata.len() < prompt_metadata.len() {
                oversized_files.push(prompt_file_name);
            }
        }
    }

    // Se houver arquivos sem documentação correspondente, exibe erro
    if !missing_docs.is_empty() {
        eprintln!("\x1b[31mFaltando documentação para os seguintes arquivos na pasta \x1b[0m\n{}: {:?}", &doc_folder, missing_docs);
        std::process::exit(1);
    }

    // Se houver arquivos de código maiores que os arquivos de documentação correspondentes, exibe erro
    if !oversized_files.is_empty() {
        eprintln!("\x1b[31mOs seguintes arquivos de código são maiores que seus correspondentes na pasta de documentação:\x1b[0m {:?}", oversized_files);
        std::process::exit(1);
    }

    // Se todas as verificações passarem, exibe uma mensagem de sucesso
    println!("\x1b[32mTodas as regras de negócio foram executadas com sucesso!\x1b[0m");

    Ok(())
}
