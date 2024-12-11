use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEventKind}, // For handling keyboard events
    execute, // To execute terminal commands
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    style::{Color, SetForegroundColor},
};
use std::{fs::read_to_string, io::stdout, io::Write}; // For file reading and standard output
use std::io;
use std::time::Instant;

// Define the App struct to hold the state of the application
struct App {
    file_content: String, // The text that the user is supposed to type
    user_input: String,   // The text entered by the user so far
}

impl App {
    fn new(file_name: &str) -> Result<Self, std::io::Error> {
        let file_content = read_to_string(file_name)?; // Read file content into a string
        Ok(Self {
            file_content,          // Assign file content
            user_input: String::new(), // Initialize user input as an empty string
        })
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut app = App::new("typing.txt")?; // Load the file content
    let start = Instant::now();

    println!("Choose an option:");
    println!("1. Simple typing tutor version");
    println!("2. 'e' remove version");

    // Read user input (raw mode disabled here)
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    // Parse input as an integer
    match choice.trim().parse::<u32>() {
        Ok(1) => {
            enable_raw_mode()?; // Enable raw mode for typing loop
            loop {
                // Clear the terminal and reset the cursor position
                execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;

                // Show the target text to the user (fixed at the top of the screen)
                println!("{}", app.file_content);

                execute!(stdout(), cursor::MoveTo(0, app.file_content.lines().count() as u16 + 1))?;
                // Iterate over both user input and target text and compare each character
                for (letter1, letter2) in app.user_input.chars().zip(app.file_content.chars()) {
                    if letter1 == letter2 {
                        // If the characters match, print the character
                        print!("{letter2}");
                    } else {
                        // If the characters don't match, print a red block
                        execute!(stdout(), SetForegroundColor(Color::Red)).unwrap();
                        print!("█"); // Red block for mismatched character
                        execute!(stdout(), SetForegroundColor(Color::Reset)).unwrap(); // Reset color
                    }
                }

                // Print a cursor
                print!("_");
                stdout().flush()?; // Ensure immediate output

                // Read the next keyboard event from the user
                if let Event::Key(key_event) = read()? {
                    if key_event.kind == KeyEventKind::Press {
                        match key_event.code {
                            KeyCode::Backspace => {
                                // If the Backspace key is pressed, remove the last character
                                app.user_input.pop();
                            }
                            KeyCode::Esc => break, // Exit the loop if Escape is pressed
                            KeyCode::Char(c) => {
                                // Only process input if it doesn't exceed the file content's length
                                if app.user_input.len() < app.file_content.len() {
                                    app.user_input.push(c);
                                }
                            }
                            KeyCode::Enter => {
                                // Calculate typing score when Enter is pressed
                                let total_chars = app.file_content.chars().count();
                                let total_right = app
                                    .user_input
                                    .chars()
                                    .zip(app.file_content.chars())
                                    .filter(|(a, b)| a == b)
                                    .count();

                                let word_count: f64 = app.user_input.trim().split_whitespace().count() as f64;

                                let words_per_minute = (word_count * 60.0) / start.elapsed().as_secs_f64();
                                // Move the cursor to the beginning of a line to print feedback
                                execute!(stdout(), cursor::MoveTo(0, app.file_content.lines().count() as u16))?;

                                println!("\nYou got {total_right} out of {total_chars}!");
                                println!("\rYou typed {:.0} words in {:.2} seconds", word_count, start.elapsed().as_secs_f64());
                                println!("\rWords per minute: {:.2}", words_per_minute);

                                execute!(stdout(), cursor::MoveTo(0, app.file_content.lines().count() as u16 + 4))?;
                                break; // Exit typing loop after displaying results
                            }
                            _ => {} // Ignore other keys
                        }
                    }
                }
            }
            disable_raw_mode()?; // Restore terminal mode after the loop
        }
        Ok(2) => {
            execute!(stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;

            println!("Please enter a sentence:");

            enable_raw_mode()?; // Enable raw mode for capturing individual keypresses

        // Clear the terminal and move cursor to the top-left corner
        execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        println!("Please start typing:");

        let mut user_input = String::new();

        loop {
            // Clear the terminal screen and reset the cursor position
            execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;

            // Display the user input
            for c in user_input.chars() {
                if c == 'e' {
                    // Display a red block for 'e'
                    execute!(io::stdout(), SetForegroundColor(Color::Red))?;
                    print!("█");
                    execute!(io::stdout(), SetForegroundColor(Color::Reset))?;
                } else {
                    // Otherwise, display the character as usual
                    print!("{}", c);
                }
            }

            // Flush the output to the terminal immediately
            io::stdout().flush()?;

            // Wait for the next key event
            if let Event::Key(key_event) = read()? {
                if key_event.kind == KeyEventKind::Press {
                    match key_event.code {
                        KeyCode::Backspace => {
                            // Remove the last character if Backspace is pressed
                            user_input.pop();
                        }
                        KeyCode::Esc => break, // Exit loop if Esc is pressed
                        KeyCode::Char(c) => {
                            // Add the typed character to the user input
                            user_input.push(c);
                        }
                        KeyCode::Enter => {
                            // Exit the loop when Enter is pressed
                            break;
                        }
                        _ => {} // Ignore other keys
                    }
                }
            }
        }

    // After typing is done, process the input
    let input_without_e_words: String = user_input
        .split_whitespace()
        .filter(|word| !word.contains('e'))
        .collect::<Vec<&str>>()
        .join(" ");

    // Display the final result after filtering words containing 'e'
    execute!(io::stdout(), Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    println!("Your input without words containing 'e':");
    execute!(stdout(), cursor::MoveTo(0, app.file_content.lines().count() as u16 + 1))?;
    println!("{}", input_without_e_words);

    disable_raw_mode()?; // Disable raw mode after finishing typing
    return Ok(())
        }
        _ => {
            println!("Invalid choice. Please enter 1 or 2.");
        }
    }

    Ok(())
}