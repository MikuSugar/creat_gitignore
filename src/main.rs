use std::fs;
use std::process::Command;
use std::collections::HashSet;
use std::path::Path;

fn main() {
    let cmd = std::env::args().nth(1).expect("no cmd,pleas execute cgi help");
    if cmd == "init" {
        init();
    } else if cmd == "ls" {
        ls();
    } else if cmd == "help" {
        help();
    } else if cmd == "add" {
        let custom_file = std::env::args().nth(2).expect("no custom_file,pleas execute cgi help");
        add(&custom_file);
    } else {
        handle(&cmd);
    }
}

fn add(custom_file: &String) {
    if let Ok(metadata) = fs::metadata(custom_file) {
        if metadata.is_file() {
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            let cgi_dir = format!("{}/.cgi", home_dir);
            let file_name = String::from(Path::new(custom_file).file_stem().unwrap().to_str().unwrap());

            let custom_path = format!("{}/custom", &cgi_dir);
            if let Err(_e) = fs::create_dir(&custom_path) {} else {
                println!("Created custom directory at {}", custom_path);
            }

            let custom_dst_path = format!("{}/custom/{}.gitignore", cgi_dir, file_name);
            fs::copy(custom_file, custom_dst_path).expect("Failed to copy gitignore template.");
        } else {
            eprintln!("{} is not a file.", custom_file);
        }
    } else {
        eprintln!("The file {} does not exist.", custom_file);
    }
}

fn handle(s: &String) {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cgi_dir = format!("{}/.cgi", home_dir);
    let file_path = format!("{}/gitignore/{}.gitignore", cgi_dir, s);
    let custom_file_path = format!("{}/custom/{}.gitignore", cgi_dir, s);


    if fs::metadata(&custom_file_path).is_ok() && fs::metadata(&custom_file_path).unwrap().is_file() {
        let dst_path = ".gitignore";
        fs::copy(custom_file_path, dst_path).expect("Failed to copy gitignore template.");
    } else {
        if fs::metadata(&file_path).is_ok() && fs::metadata(&file_path).unwrap().is_file() {
            let dst_path = ".gitignore";
            fs::copy(file_path, dst_path).expect("Failed to copy gitignore template.");
        } else {
            eprintln!("{} not found ! You may need to execute the init command to initialize or execute ls to view the list of existing templates.", s);
        }
    }
}

fn init() {
    println!("init...");
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cgi_dir = format!("{}/.cgi", home_dir);
    if let Err(_e) = fs::create_dir(&cgi_dir) {} else {
        println!("Created .cgi directory at {}", cgi_dir);
    }

    let custom_path = format!("{}/custom", &cgi_dir);
    if let Err(_e) = fs::create_dir(&custom_path) {} else {
        println!("Created custom directory at {}", custom_path);
    }

    let dir_path = format!("{}/gitignore", &cgi_dir);
    if let Err(_e) = fs::remove_dir_all(&dir_path) {} else {
        println!("Directory deleted: {}", dir_path);
    }
    let cmd = format!("cd {} && git clone https://github.com/github/gitignore.git", &cgi_dir);
    Command::new("bash")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap();
    println!("init success.");
}

fn ls() {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cgi_dir = format!("{}/.cgi", home_dir);
    let dir_path = format!("{}/gitignore", cgi_dir);
    let ext = "gitignore";

    let mut set = HashSet::new();
    traverse(dir_path, ext, &mut set);
    let custom_path = format!("{}/custom", &cgi_dir);
    traverse(custom_path, ext, &mut set);

    for item in &set {
        println!("{}", item);
    }
}

fn traverse(dir_path: String, ext: &str, set: &mut HashSet<String>) {
    let entries = fs::read_dir(dir_path).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().unwrap_or((&"").as_ref()) == ext {
            let file_name = String::from(path.file_stem().unwrap().to_str().unwrap());
            set.insert(file_name);
        }
    }
}


fn help() {
    println!("cgi init: Initialize and download the template library from GitHub(https://github.com/github/gitignore.git).");
    println!("cgi ls: View the list of templates.");
    println!("cgi add a.gitignore: Add a.gitignore to the custom template library.");
    println!("If you want to add a template file of rust, you should execute: cgi Rust.");
}
