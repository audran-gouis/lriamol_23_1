// Import necessary libraries from crossterm and std
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind}, // For handling keyboard events
    execute, // To execute terminal commands
    terminal::{Clear, ClearType}, // To clear the terminal screen
};
use std::{fs::read_to_string, io::stdout}; // For file reading and standard output

// Define the App struct to hold the state of the application
struct App {
    file_content: String, // The text that the user is supposed to type
    user_input: String,   // The text entered by the user so far
}

// Implement methods for the App struct
impl App {
    // Create a new instance of App by reading the file content and initializing user input
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(file_name)?; // Read file content into a string
        Ok(Self {
            file_content, // Assign file content
            user_input: String::new(), // Initialize user input as an empty string
        })
    }
}

// Main entry point of the program
fn main() -> Result<(), std::io::Error> {
    let mut app = App::new("typing.txt")?; // Create an App instance by loading the "typing.txt" file
    
    loop {
        // Show the target text to the user
        println!("{}", app.file_content); // Print the content that the user must type        
        
        // Iterate over both user input and target text and compare each character
        for (letter1, letter2) in app.user_input.chars().zip(app.file_content.chars()) {
            if letter1 == letter2 { // If the characters match, print the character
                print!("{letter2}");
            } else { // If the characters don't match, print an asterisk
                print!("*");
            }
        }
        println!("_"); // Print an underscore to represent the cursor position

        // Read the next keyboard event from the user
            if let Event::Key(key_event) = read()? {
                if key_event.kind == KeyEventKind::Press { // Check if the key was pressed (not released)
                    match key_event.code {
                        KeyCode::Backspace => { // If the Backspace key is pressed, remove the last character
                            app.user_input.pop();
                        }

                        KeyCode::Esc => break, // If the Esc key is pressed, exit the loop (end the program)
                        
                        KeyCode::Char(c) => { // If any character key is pressed, append it to the user input
                            app.user_input.push(c);
                        }

                        KeyCode::Enter => { // If the Enter key is pressed, calculate the typing score
                            let total_chars = app.file_content.chars().count(); // Count the total characters in the target text
                            let total_right = app
                                .user_input
                                .chars()
                                .zip(app.file_content.chars()) // Compare each character of user input with the target text
                                .filter(|(a, b)| a == b) // Count how many characters match
                                .count();
                            println!("You got {total_right} out of {total_chars}!"); // Display the score
                            return Ok(()); // End the program
                        }
                        _ => {} // For any other keys, do nothing
                    }
                }
            }

                // Clear the terminal screen for the next loop iteration
                execute!(stdout(), Clear(ClearType::All))?; // Clear the screen
        }
    
        Ok(()) // Return Ok when the program finishes
}
