use std::env;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        let file_path = &args[i];
        let contents = match fs::read_to_string(file_path) {
            Ok(c) => c,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("cat: {file_path}: No such file or directory"),
                ErrorKind::PermissionDenied => panic!("cat: {file_path}: Access denied"),
                other_error => {
                    if Path::new(file_path).is_dir() {
                        panic!("cat: {file_path}: Is a directory");
                    }
                    panic!("cat: {file_path}: Problem opening the file, {:?}", other_error);
                }
            }
        };
        
        println!("{contents}");
        
        i += 1;
    }
}
