use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn run_command(args: &[String]) -> io::Result<()> {
    // Check if all necessary arguments for the 'run' action were provided
    if args.len() < 5 {
        eprintln!("\x1b[31mError: Project name, language model, and file extension are required for the 'run' action.\x1b[0m");
        std::process::exit(1);
    }

    let project_name = &args[2];
    let extension = &args[3];
    let model_language = &args[4];

    // Paths to prompt and documentation folders
    let prompt_folder = format!("projects/{}/auto/prompt/{}/", project_name, model_language);
    let doc_folder = format!("projects/{}/doc/", project_name);

    // Check if the documentation folder exists
    if !Path::new(&doc_folder).exists() {
        eprintln!("\x1b[31mMissing documentation in the {} folder in the doc/ directory.\x1b[0m", prompt_folder);
        std::process::exit(1);
    }

    // Check if there are files in the prompt folder with the provided extension
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
            eprintln!("\x1b[31mError accessing the code folder.\x1b[0m");
            std::process::exit(1);
        }
    };

    // Check if there are files in the code folder with the specified extension
    if prompt_files.is_empty() {
        eprintln!("\x1b[31mNo files with the '{}' extension were found in the code folder.\x1b[0m", extension);
        std::process::exit(1);
    }

    // Check if there are corresponding files in the documentation folder
    let mut missing_docs = Vec::new();
    let mut oversized_files = Vec::new();
    for prompt_file in prompt_files {
        let prompt_file_name = prompt_file.file_name().unwrap().to_string_lossy().to_string();
        // Path to the corresponding documentation file
        let doc_file_name = format!("{}.md", &prompt_file_name);
        let doc_path = Path::new(&doc_folder).join(&doc_file_name);

        // Check if the documentation file exists
        if !doc_path.exists() {
            missing_docs.push(prompt_file_name);
        } else {
            // Check the size of the documentation file compared to the prompt file
            let prompt_metadata = fs::metadata(&prompt_file).unwrap();
            let doc_metadata = fs::metadata(&doc_path).unwrap();
            if doc_metadata.len() < prompt_metadata.len() {
                oversized_files.push(prompt_file_name);
            }
        }
    }

    // If there are files without corresponding documentation, display error
    if !missing_docs.is_empty() {
        eprintln!("\x1b[31mMissing documentation for the following files in the {} folder:\x1b[0m\n{:?}", &doc_folder, missing_docs);
        std::process::exit(1);
    }

    // If there are code files larger than their counterparts in the documentation folder, display error
    if !oversized_files.is_empty() {
        eprintln!("\x1b[31mThe following code files are larger than their counterparts in the documentation folder:\x1b[0m {:?}", oversized_files);
        std::process::exit(1);
    }

    // If all checks pass, display a success message
    println!("\x1b[32mAll business rules were successfully executed!\x1b[0m");

    Ok(())
}
