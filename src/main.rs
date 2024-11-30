use std::env; 
use std::fs;
use std::path::Path;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Error forgot: <foldername> <framework> <language>"); 
        std::process::exit(0)
    }

    let path = format!("./src/boilerplate/{}/{}", &args[2], &args[3]);

    match fs::read_dir(path.clone()) {
        Ok(_) => {
            create_folder(&args[1]);
            copy_dir(path, &args[1]).expect("ERROR");
        },
        Err(_) => {
            eprintln!("Did not find path the current path: {}", path);
            std::process::exit(0)
        }
    }
}

fn create_folder(foldername: &str) {
    if !std::path::Path::new(foldername).exists() {
        let _ = fs::create_dir(foldername);
    } else {
        println!("Folder named {}, already exists", foldername);
        std::process::exit(0)
    }
}

fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}