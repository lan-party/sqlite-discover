use std::env;
use std::fs;
use std::str;
use std::io::Read;
use glob::glob;

fn main() {
    // Recursively search a given directory for files starting with 'SQLite format 3'
    
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {

        // Get search directory path from argument
        let search_dir: String = args[1].clone().parse().unwrap();

        // Recursively search for all files
        for entry in glob(&format!("{}\\**\\*", search_dir)).expect("glob err") {

            // Get file/folder path from glob iter
            let path  = match entry {
                Ok(p) => p,
                Err(e) => "".into(),
            };

            // Check if it's a file
            if fs::metadata(path.clone()).unwrap().is_file() {

                // Get first 15 bytes
                let mut f = fs::File::open(path.clone()).expect("fs::File::open err");
                let mut buf = [0; 15];
                f.read(&mut buf).expect("read err");

                let check_str = match str::from_utf8(&buf) {
                    Ok(v) => v,
                    Err(e) => "",
                };

                if check_str == "SQLite format 3" {
                    println!("{}", path.display());
                }
            }
        }

    }else{

        println!("Usage:\n\tsqlite-discover [directory]");
    }
}
