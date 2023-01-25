use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let query     = &args[1];
    let file_path = &args[2];
    println!("Search {} in {}", query, file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("{}", &contents);
}
