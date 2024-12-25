use std::{error::Error, fs, io, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let root_folder = "/home/suntu/assets/";
    // let root_dir = Path::new(root_folder);
    // println!()
    let folders = read_folders_in_folder(root_folder)?;
    println!("Folders in the specified folder:");
    for folder in folders {
        println!("{}", folder);
        copy_and_rename_json_files(&folder)?;
    }

    Ok(())
}

fn read_folders_in_folder(folder_path: &str) -> io::Result<Vec<String>> {
    let mut folders = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        println!("entry {:?}", entry.path());
        let string = entry.path().display().to_string();
        folders.push(string);
    }

    Ok(folders)
}

fn copy_and_rename_json_files(folder_path: &str) -> io::Result<()> {
    let folder_path = Path::new(folder_path);

    // Check if the folder exists
    if !folder_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Folder not found"));
    }

    // Determine the folder name to use for renaming
    let folder_name = folder_path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid folder name"))?;

    // Iterate over files in the folder
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();

        // Process only files with `.json` extension
        if path.is_file() && path.extension().map_or(false, |ext| ext == "json") {
            let new_file_name = format!("{}.json", folder_name);
            let new_path = folder_path.join(new_file_name);

            // Copy and rename the file
            fs::copy(&path, &new_path)?;
            println!(
                "Copied and renamed: {} -> {}",
                path.display(),
                new_path.display()
            );
        }

        // Process only files with `.jpg` extension
        if path.is_file() && path.extension().map_or(false, |ext| ext == "jpg") {
            let new_file_name = format!("{}.jpg", folder_name);
            let new_path = folder_path.join(new_file_name);

             // Remove existing file if it exists
            if new_path.exists() {
                fs::remove_file(&new_path)?;
                println!("Removed old file: {}", new_path.display());
            }

            // Copy and rename the file
            fs::copy(&path, &new_path)?;
            println!(
                "Copied and renamed: {} -> {}",
                path.display(),
                new_path.display()
            );
        }
    }

    Ok(())
}


