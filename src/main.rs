use std::io; 

fn main() {
    println!("Hello, world!");

   
    println!("Please enter something:");


    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {
      
            println!("You entered: {}", input);
        }
        Err(error) => {
            eprintln!("Error reading input: {}", error);
        }
    }
}
