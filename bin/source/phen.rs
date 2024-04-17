use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() -> io::Result<()> {
    // Verifica se foi fornecido um nome de projeto como argumento
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Erro: É necessário fornecer um nome de projeto como argumento.");
        std::process::exit(1);
    }

    // Nome do projeto fornecido como argumento
    let nome_do_projeto = &args[1];

    // Cria a estrutura de pastas dentro da pasta do projeto
    let project_path = Path::new("projects").join(nome_do_projeto);
    fs::create_dir_all(project_path.join("manual"))?;
    fs::create_dir_all(project_path.join("auto/prompt/GPT-3.5_v1"))?;
    fs::create_dir_all(project_path.join("auto/prompt/GPT-4.0_v2"))?;
    fs::create_dir_all(project_path.join("doc"))?;
    fs::create_dir_all(project_path.join("framework"))?;

    // Cria arquivos de exemplo nas pastas Prompt
    fs::File::create(project_path.join("auto/prompt/GPT-3.5_v1/files_Prompt_1.txt"))?;
    fs::File::create(project_path.join("auto/prompt/GPT-3.5_v1/files_Prompt_2.txt"))?;
    fs::File::create(project_path.join("auto/prompt/GPT-4.0_v2/files_Prompt_1.txt"))?;
    fs::File::create(project_path.join("auto/prompt/GPT-4.0_v2/files_Prompt_2.txt"))?;

    // Cria arquivos de exemplo nas pastas doc
    fs::File::create(project_path.join("doc/files_Prompt_1.txt.md"))?;
    fs::File::create(project_path.join("doc/files_Prompt_2.txt.md"))?;


    println!("Estrutura de pastas para o projeto 'projects/{}' criada com sucesso!", nome_do_projeto);

    Ok(())
}
