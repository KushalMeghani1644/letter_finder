use std::io;
fn main() {
    //Rust code for taking a string from user and finding a letter in it based on users input
    let mut usr_string = String::new();
    println!("Enter a string: ");
    io::stdin()
        .read_line(&mut usr_string)
        .expect("Please enter a valid string");
    let mut usr_letter = String::new();
    println!("Enter a letter to find: ");
    io::stdin()
        .read_line(&mut usr_letter)
        .expect("Please enter a valid letter");
    let usr_letter = usr_letter.trim();
    if usr_letter.len() != 1 {
        println!("Please enter a single letter");
        return;
    }
    let usr_string = usr_string.trim();
    let mut found = false;
    let mut position = Vec::new();
    for (i, c) in usr_string.chars().enumerate() {
        if c.to_string() == usr_letter {
            found = true;
            position.push(i);
        }
    }
    if found {
        println!(
            "The letter '{}' was found at positions: {:?}",
            usr_letter, position
        );
    } else {
        println!("The letter '{}' was not found in the string", usr_letter);
    }
}
