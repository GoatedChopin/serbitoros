use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path;
use std::path::PathBuf;
use std::vec;
use tera::Tera;
use tera::Context;

// might replace with the generate_indexes.py script using ct_python! rust macro

pub fn main() -> () {
    let static_path = "./src/static";
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    generate_index(&static_path, &tera);
}

fn generate_index(directory: &str, tera: &Tera) -> () {
    let mut path_vec = Vec::new();
    let mut dir_vec = Vec::new();
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("{:?} is a dir", path);
            generate_index(&path.to_string_lossy(), &tera);
            dir_vec.push(path);
        } else if path.ends_with("index.html"){
            println!("Index file");
        } else {
            println!("{:?} is a file", path);
            path_vec.push(path);
        }
    }
    let mut context = Context::new();
    context.insert("files", &path_vec);
    context.insert("dirs", &dir_vec);
    println!("{}/index.html", directory);
    let html = tera.render(&format!("{}/index.html", directory), &context).unwrap();
    write_index(&format!("{}/index.html", directory), &html);
    // Tera::one_off(input, context, autoescape)
}

fn write_index(filepath: &str, html: &str) -> std::io::Result<()> {
    let final_path = format!("final_{}", filepath);
    let mut output = File::create(final_path)?;
    output.write_all(html.as_bytes());
    // write!(output, "{}", html);
    Ok(())
}