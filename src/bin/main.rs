use std::{error::Error, fs, io, path::Path};

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

fn read_image_files(folder_path: &str) -> io::Result<Vec<String>> {
    let mut image_files = Vec::new();

    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy().to_lowercase(); // Convert to lowercase for case-insensitivity

        // Check if the file extension indicates it's an image file
        if file_name_str.ends_with(".jpg")
            || file_name_str.ends_with(".jpeg")
            || file_name_str.ends_with(".png")
            || file_name_str.ends_with(".gif")
        {
            image_files.push(entry.path().display().to_string());
        }
    }

    Ok(image_files)
}

fn convert_to_webp(file_path: &str) -> io::Result<()> {
    let input_path = Path::new(file_path);
    let output_path = input_path.with_extension("webp");
    std::process::Command::new("cwebp")
        .arg(input_path)
        .arg("-o")
        .arg(output_path)
        .output()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let root_folder = "/home/suntu/assets/";
    // let root_dir = Path::new(root_folder);
    // println!()
    let folders = read_folders_in_folder(root_folder)?;
    println!("Folders in the specified folder:");
    for folder in folders {
        println!("{}", folder);
        let image_files = read_image_files(folder.as_str())?;
        for image_path in image_files {
            println!("image_path {}", image_path);
            convert_to_webp(&image_path)?;
        }
    }

    Ok(())
}
