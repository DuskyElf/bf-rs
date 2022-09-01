use std::{env, process::exit, fs::File, io::Read};

fn main() {
    let file_path = ui();
    let source = load_porgram(file_path);
    println!("{}", source);
}

fn ui() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: bf-rs <file.bf>");
        exit(400);
    }

    args[1].clone()
}

fn load_porgram(file_path: String) -> String {
    let mut file = match File::open(file_path) {
        Ok(t) => t,
        Err(e) => {
            println!("Error: {}", e);
            exit(404);
        }
    };
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Failed to read the file.");
    source
}