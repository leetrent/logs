use std::fs;

fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }
    results
}

fn main() {
    match fs::read_to_string("logs.txt") {
        Ok(read_text) => {
            let error_logs = extract_errors(read_text.as_str());
            println!("{:#?}", error_logs);
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
