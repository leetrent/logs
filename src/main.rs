use std::fs;
use std::io::Error;

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

fn main() -> Result<(), Error> {

    ////////////////////////////////////////////////////////////////////
    // SUCCESS CONDITION
    ////////////////////////////////////////////////////////////////////
    // let text = fs::read_to_string("logs.txt")?;
    // let error_logs = extract_errors(text.as_str());
    // fs::write("errors.txt", error_logs.join("\n"))?;
    // Ok(())

    ////////////////////////////////////////////////////////////////////
    // FAILURE CONDITION
    ////////////////////////////////////////////////////////////////////
    let text = fs::read_to_string("no_such_file.txt")?; // '?' is the try operator
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;
    Ok(())
}

