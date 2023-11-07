use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Output};
use std::thread;
use std::time::Duration;
use std::process::{Stdio};

fn main() {
    // Wait for 1 seconds
    thread::sleep(Duration::from_secs(1));

    let mut file = File::create("clipclap.txt").expect("Failed to create file");

    let output = Command::new("xsel")
        .arg("-p")
        .stdout(Stdio::from(file.try_clone().expect("Failed to clone file")))
        .output()
        .expect("Failed to execute xsel command");

    if !output.status.success() {
        eprintln!("Failed to execute xsel command");
        std::process::exit(1);
    }


    show_tooltip("Gestartet.");


    // Generate a random 6-digit number
    let random_number: u32 = rand::thread_rng().gen_range(100_000..=999_999);

    // Create the file name with the prefix "snip" and the random number
    let file_name = format!("snip{}.md", random_number);

    // Expand the base folder path
    let base_folder = shellexpand::tilde("~/Nextcloud2/Obsy").into_owned();

    // Create the full file path
    let file_path = Path::new(&base_folder).join(&file_name);

    // Save clipboard content to file
    let mut file = File::create(&file_path).unwrap();
    file.write_all(get_clipclap_content().as_bytes()).unwrap();
    println!("content: {}", get_clipclap_content());

    println!("File created: {:?}", file_path);

    show_tooltip(&get_clipclap_content());
    
}

fn display_tooltip(message: &str, duration: u32) -> Result<Output, std::io::Error> {
    let output = Command::new("kdialog")
        .args(&["--passivepopup", message, &duration.to_string()])
        .output();

    output
}

fn get_clipclap_content() -> String {
    let mut content = String::new();
    let mut file = File::open("clipclap.txt").expect("Failed to open file");
    file.read_to_string(&mut content).expect("Failed to read file");
    
    println!("len of content: {}", content.len());
    // if len content = 0 then stop program
    if content.len() == 0 {
        std::process::exit(1);
    }

    content   
}

fn show_tooltip(message: &str) {
    let result = display_tooltip(message, 2000);
    match result {
        Ok(output) => {
            if output.status.success() {
                println!("Tooltip displayed successfully.");
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                println!("Failed to display tooltip: {}", stderr);
            }
        }
        Err(error) => {
            println!("Failed to execute kdialog command: {}", error);
        }
    }
}