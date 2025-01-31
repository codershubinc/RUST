fn main() {
    let mut random_number: i32 = 0; // Declare a mutable variable to store the random number
    let mut guess: i32 = 0; // Declare a mutable variable to store the player's guess

    // Import the rand crate for random number generation
    use rand::Rng; // This gives access to the random number generation functions

    // Generate a random number between 1 and 100 (inclusive)
    random_number = rand::thread_rng().gen_range(1..101); // Use the thread_rng() method to generate a random number in the given range

    println!("Guess the number!"); // Prompt the user to guess a number

    // Start a loop that will continue until the player guesses correctly

    loop {
        println!("Please input your guess."); // Ask for the player's guess

        let mut input = String::new(); // Declare a mutable string variable to hold the input from the user
                                       // Read the input from stdin (standard input) and store it in the `input` variable
        std::io::stdin()
            .read_line(&mut input) // Read a line from the user
            .expect("Failed to read line"); // Handle any potential errors while reading input

        // Try to parse the input into an integer (i32)
        guess = match input.trim().parse() {
            Ok(num) => num, // If successful, use the parsed number as the guess
            Err(_) => {
                // If parsing fails, print an error message and continue the loop
                println!("Please enter a number!");
                continue; // Skip this iteration and continue asking for a guess
            }
        };

        println!("You guessed: {}", guess); // Show the user what they guessed

        // Compare the guess with the random number
        match guess.cmp(&random_number) {
            // `cmp` compares the two numbers and returns an Ordering
            std::cmp::Ordering::Less => println!("Too small!"), // If guess is less than random_number
            std::cmp::Ordering::Greater => println!("Too big!"), // If guess is greater than random_number
            std::cmp::Ordering::Equal => {
                // If guess equals the random_number
                println!("You win!"); // Print a winning message
                break; // Exit the loop
            }
        }
    }
}
