use std::fs;
use std::process::Command;

fn main() {
    let cmd = std::env::args().nth(1).expect("no cmd,pleas execute cgi help");
    if cmd == "init" {
        init();
    } else if cmd == "ls" {
        ls();
    } else if cmd == "help" {
        help();
    } else {
        handle(&cmd)
    }
}

fn handle(s: &String) {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cgi_dir = format!("{}/.cgi", home_dir);
    let file_path = format!("{}/gitignore/{}.gitignore", cgi_dir, s);

    if fs::metadata(&file_path).is_ok() && fs::metadata(&file_path).unwrap().is_file() {
        let dst_path = ".gitignore";
        fs::copy(file_path, dst_path).expect("Failed to copy gitignore template.");
    } else {
        eprintln!("{} not found ! You may need to execute the init command to initialize or execute ls to view the list of existing templates.", s);
    }
}

fn init() {
    println!("init...");
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let cgi_dir = format!("{}/.cgi", home_dir);
    if let Err(_e) = fs::create_dir(&cgi_dir) {} else {
        println!("Created .cgi directory at {}", cgi_dir);
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
    let entries = fs::read_dir(dir_path).unwrap();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.is_file() && path.extension().unwrap_or((&"").as_ref()) == ext {
            let file_name = path.file_stem().unwrap().to_str().unwrap();
            println!("{}", file_name);
        }
    }
}


fn help() {
    println!("cgi init: Initialize and download the template library from GitHub(https://github.com/github/gitignore.git).");
    println!("cgi ls: View the list of templates.");
    println!("If you want to add a template file of rust, you should execute: cgi Rust");
}
