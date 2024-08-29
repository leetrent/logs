use std::fs;

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(read_text) => {
            println!("# of characters read: {}", read_text.len());
        },
        Err(err_msg) => {
            println!("{}", err_msg);
        }
    }

    match fs::read_to_string("no_file.txt") {
        Ok(read_text) => {
            println!("# of characters read: {}", read_text.len());
        },
        Err(err_msg) => {
            println!("{}", err_msg);
        }
    }
}
