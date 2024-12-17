use std::fs::read_dir;
use std::path::Path;

pub fn list_files(target: &String) {
    let target_path = Path::new(&target);
    match read_dir(target_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => println!("{}", entry.path().display()),
                    Err(error) => println!("{} <E>>", error),
                }
            }
        }
        Err(error) => {
            eprintln!("{}", error)
        }
    }
}
