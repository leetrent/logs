use std::fs;

fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");
    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}

fn main() {
    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(read_text) => {
            error_logs = extract_errors(read_text.as_str());
        },
        Err(err_msg) => {
            println!("{}", err_msg);
        }
    }

    println!("{:#?}", error_logs);

    match fs::read_to_string("no_file.txt") {
        Ok(read_text) => {
            println!("# of characters read: {}", read_text.len());
        },
        Err(err_msg) => {
            println!("{}", err_msg);
        }
    }
}
