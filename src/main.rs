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
            match fs::write("errors.txt", error_logs.join("\n")) {
                Ok(..) => println!("Successfully wrote errors to errors.txt file"),
                Err(error_message) => println!("Attempt to write errors to errors.txt file FAILED. Reason: {}", error_message)
            }
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
