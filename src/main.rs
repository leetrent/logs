use std::fs;

fn string_test (a: String, b: &String, c: &str) {

}


fn main() {

    string_test (
        String::from("red"),
        &String::from("red"),
        "red"
    );

    string_test (
        "red".to_string(),
        &String::from("red"),
        String::from("red").as_str()
    );

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
