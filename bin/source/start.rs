use std::fs;
use std::io;
use std::path::Path;

pub fn create_project_structure(nome_do_projeto: &str) -> io::Result<()> {
    // Cria a estrutura de pastas dentro da pasta do projeto
    let project_path = Path::new("projects").join(nome_do_projeto);
    fs::create_dir_all(project_path.join("stable"))?;
    fs::create_dir_all(project_path.join("auto/prompt/gpt3"))?;
    fs::create_dir_all(project_path.join("auto/prompt/gpt4"))?;
    fs::create_dir_all(project_path.join("doc"))?;
    fs::create_dir_all(project_path.join("framework"))?;

    // Cria arquivos de exemplo nas pastas Prompt
    fs::File::create(project_path.join("auto/prompt/gpt3/files_Prompt_1.txt"))?;
    fs::File::create(project_path.join("auto/prompt/gpt3/files_Prompt_2.txt"))?;
    fs::File::create(project_path.join("auto/prompt/gpt4/files_Prompt_1.txt"))?;
    fs::File::create(project_path.join("auto/prompt/gpt4/files_Prompt_2.txt"))?;

    // Cria arquivos de exemplo nas pastas doc
    fs::File::create(project_path.join("doc/files_Prompt_1.txt.md"))?;
    fs::File::create(project_path.join("doc/files_Prompt_2.txt.md"))?;

    println!("Estrutura de pastas para o projeto 'projects/{}' criada com sucesso!!!", nome_do_projeto);


    Ok(())
}
