use std::io::Error;

// Todo: add in a return type
fn validate_ingredients(ingredients: &Vec<String>) -> Result<(), Error>{
    if ingredients.len() > 3 {
        Err(Error::other("A maximum of 3 ingredients are allowed."))
    } else {
        Ok(())
    }
}

fn main() {
    let ingredients = vec![
        String::from("Cheese"),
        String::from("Tomatoes"),
        String::from("Peppers"),
        String::from("Olives"),
    ];
    
    // Todo: validation is an operation that might succeed or fail
    // Print out a success or fail message based on whether
    // it passes validation
    match validate_ingredients(&ingredients) {
        Ok(..) => println!("List contains a valid number of ingredients."),
        Err(err_msg) => println!("{}", err_msg)
    }
}







