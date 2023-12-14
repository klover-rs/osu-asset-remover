use std::fs;
use std::path::PathBuf;
use std::io::stdin;
use std::process;
use ansi_term::Colour;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    println!("enter the full path of your osu! directory\n\ne.g.: C:\\Users\\rosie\\AppData\\Local\\osu!");

    let mut buffer = String::new();

    stdin().read_line(&mut buffer)?;
    let filepath: String = match buffer.trim_end() {
        "" => {
            println!("You should enter a path silly\nexited code with 0");
            process::exit(0);
        }
        path => format!("{path}"),
    };

    println!("we got this file path as an output: {}", filepath);

    println!("{}", Colour::Red.bold().paint("ARE YOU SURE THAT YOU WANT TO DELETE ALL ASSETS OF YOUR BEATMAPS FOLDER? (Y/N)"));

    let mut buffer1 = String::new();

    stdin().read_line(&mut buffer1).expect("Failed to read line");

    let result: String = match buffer1.trim_end() {
        "" => {
            println!("exited code with 0");
            process::exit(0);
        }
        "Y" => {
            println!("as you wish..");
            String::from("Y")
        }
        "N" => {
            println!("you aborted the action.\nexited code with 0");
            process::exit(0);
        }
        "y" => {
            println!("as you wish..");
            String::from("Y")
        }
        "n" => {
            println!("you aborted the action.\nexited code with 0");
            process::exit(0);
        }
        _ => {
            println!("Invalid input");
            process::exit(1);
        }
    };

    let _ = result;

    match fs::metadata(format!("{}/osu!.exe", filepath)) {
        Ok(metadata) => {
            if metadata.is_file() {
                println!("found osu!.exe");

                let mut subfolder_count = 0;
                let mut deleted_file_count = 0;
                let song_folder = PathBuf::from(format!("{}", filepath)).join("Songs");
                if let Ok(entries) = fs::read_dir(song_folder) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                                let subfolder_path = entry.path();
                                println!("Subfolder: {}\n", subfolder_path.display());
                                subfolder_count += 1;

                                if let Ok(subfolder_entries) = fs::read_dir(subfolder_path) {
                                    for subfolder_entry in subfolder_entries {
                                        if let Ok(subfolder_entry) = subfolder_entry {


                                            if subfolder_entry.metadata().map(|m| m.is_file()).unwrap_or(false) {
                                                let file_extension = subfolder_entry.path().extension().map(|ext| ext.to_string_lossy().to_lowercase());
                                                if file_extension == Some("jpg".into()) || file_extension == Some("png".into()) {
                                                    let file_path = subfolder_entry.path();
                                                    println!("\nDeleting file: {}", file_path.display());
                                                    if let Err(err) = fs::remove_file(&file_path) {
                                                        eprintln!(" \n Error deleting file: {}", err);
                                                    }
                                                    deleted_file_count += 1;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    eprintln!("Error reading subfolder");
                                }
                            }
                        }
                    }
                } else {
                    eprintln!("Error reading directory: {}",  filepath);
                    process::exit(0);
                }

                println!("\nfound {} subfolders", subfolder_count);
                println!("\ndeleted {} files", deleted_file_count);
            }
        }
        Err(e) => println!("An error occurred: {}", e),
    }

    Ok(())
}
