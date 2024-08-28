use std::fs;
use std::io::Error;

fn main() {
    // let text = fs::read_to_string("logs.txt");
    // println!("{:#?}", text);
    
    match divide(5.0, 3.0) {
        Ok(result_of_division) => {
            println!("\n\nResult of Division: {}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("What went wrong: {}", what_went_wrong);
        }
    }

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("\n\nResult of Division: {}", result_of_division);
        }
        Err(what_went_wrong) => {
            println!("What went wrong: {}", what_went_wrong);
        }
    }

    match validate_email(String::from("casey@casey.com")) {
        Ok(..) => println!("\nEmail address is valid"),
        Err(error_message) => {
            println!("\nValidation Error Message: {}", error_message)
        }
    }

    match validate_email(String::from("caseycasey.com")) {
        Ok(..) => println!("\nEmail address is valid"),
        Err(error_message) => {
            println!("\nValidation Error Message: {}", error_message)
        }
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Cannot divide by zero."))
    } else {
        Ok (a / b)
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Email addresses must have an ampersand."))
    }

}
