use std::env;
use std::path::Path;
use std::io::prelude::Read;
use std::fs::File;

/// Searches the file contents for given keyword.
/// If keyword is found, file path is printed.
fn search_file(keyword: &String, path: &Path) {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {
            if contents.contains(keyword) {
                println!("> {}", path.display());
            }
        },
        Err(_) => {}
    }
}

/// Recursively searches a folder, and it's sub-folders.
/// When a file is found, search_file will be called.
fn search_folder(keyword: &String, folder: &Path) {
    match folder.read_dir() {
        Ok(entires) => {
            for entry in entires {
                let path = entry.unwrap().path();

                if path.is_file() {
                    search_file(&keyword, &path);
                } else {
                    search_folder(&keyword, &path);
                }
            }
        },
        Err(_) => {}
    }
}

fn main() {
    // check they supplied enough args
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("\nUsage: scan [keyword] [path]\n");
        return;
    }

    // parse the args
    let keyword = args.get(1).unwrap();
    let start_folder = Path::new(args.get(2).unwrap());

    // start scanning
    search_folder(&keyword, &start_folder);
}