use std::fs;
use std::io;
use std::path::Path;

pub fn create_project_structure(project_name: &str) -> io::Result<()> {
    // Create folder structure within the project directory
    let project_path = Path::new("projects").join(project_name);
    fs::create_dir_all(project_path.join("stable"))?;
    fs::create_dir_all(project_path.join("auto/prompt/gpt3"))?;
    fs::create_dir_all(project_path.join("auto/prompt/gpt4"))?;
    fs::create_dir_all(project_path.join("doc"))?;
    fs::create_dir_all(project_path.join("framework"))?;

    // Create example files in Prompt folders
    create_example_files(project_name, "gpt3")?;
    create_example_files(project_name, "gpt4")?;

    // Create example files in doc folders
    create_doc_files(project_name)?;

    println!("Folder structure for the project 'projects/{}' created successfully!", project_name);

    Ok(())
}

fn create_example_files(project_name: &str, model: &str) -> io::Result<()> {
    let prompt_folder = format!("projects/{}/auto/prompt/{}/", project_name, model);
    fs::File::create(prompt_folder.clone() + "files_Prompt_1.txt")?;
    fs::File::create(prompt_folder.clone() + "files_Prompt_2.txt")?;
    Ok(())
}

fn create_doc_files(project_name: &str) -> io::Result<()> {
    let doc_folder = format!("projects/{}/doc/", project_name);
    fs::File::create(doc_folder.clone() + "files_Prompt_1.txt.md")?;
    fs::File::create(doc_folder.clone() + "files_Prompt_2.txt.md")?;
    Ok(())
}
