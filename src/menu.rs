
// Local imports
use crate::{
    ascii::ART, db_handlers
};

// Connection import
use diesel::SqliteConnection;

// stdio - for getting input and exitting
use std::{
    io, 
    io::Write, 
    process::exit
};


// Standard home menu - also main app loop starts here
pub fn normal_menu(conn: &mut SqliteConnection) {

    // Prints the logo
    println!("{}", ART);

    // main loop
    loop {

        // input buffer
        let mut line = String::new();

        print_menu_options();

        // flush required here as it sometimes doesnt print properly
        print!("$> ");
        _ = io::stdout().flush();

        // Read input
        io::stdin().read_line(&mut line).unwrap();

        // Checks input, redirecting to the command requested
        match line.trim() {
            "a" | "add" => add(conn),
            "l" | "list" => list_todo(conn),
            "c" | "complete" => new_complete(conn),
            "r" | "remove" => remove_completed(conn),
            "o" | "options" => options(),
            "q" | "quit" => exit(0),
            _ => println!("Unknown command!")
        }

    }

}


fn add(conn: &mut SqliteConnection) {
    let mut new_message = String::new();

    println!("\nPlease enter the text for your new 'todo' item:");

    io::stdin().read_line(&mut new_message).unwrap();

    db_handlers::create_todo_item(conn, new_message);
}

pub fn list_todo(conn: &mut SqliteConnection) {
    let todo_list = db_handlers::get_todo_items(conn);

    println!("You have: {} items in your todo list", todo_list.len());

    // Styles
    let strikethrough = ansi_term::Style::new().strikethrough();

    for item in todo_list {
        print!("Item: {} | ", item.id);

        // if item is completed - strikethrough the text, else print normally
        match item.completed {
            true => println!("{}", strikethrough.paint(item.message)),
            false => println!("{}", item.message)
        }

    }
}

fn new_complete(conn: &mut SqliteConnection) {
    let todo_id: i32;

    // Loop till quit or valid i32 input
    loop {

        let mut buffer = String::new();

        println!("\nPlease enter the id of your todo item to complete");

        print!("> ");
        _ = io::stdout().flush();

        // Get input and check if it's a valid i32
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim() {
            "q" | "quit" => return,
            a => {  // If input was not 'q' or 'quit' check if it is a valid i32
                match a.parse::<i32>() {
                    Ok(i) => {
                        todo_id = i; 
                        break;
                    },
                    Err(i) => {
                        println!("Error: '{}' is not a valid i32\n
                        If you want to return type: 'q' or 'quit'", i);
                    }
                }
            }
        }

    }

    db_handlers::set_todo_complete(conn, todo_id);
}

fn remove_completed(conn: &mut SqliteConnection) {
    db_handlers::remove_completed(conn);

    println!("Completed items deletetd!");
}

fn options() {
    println!("this is on my todo list :)");
}


/* ------------- */
fn print_menu_options() {
    // Text styles
    let bold = ansi_term::Style::new().bold();
    let bold_underline = ansi_term::Style::new().bold().underline();

    // Menu options
    println!("\n{}", bold_underline.paint("Menu options:"));
    println!("{}{} - Add a new item to your todo list", 
        bold_underline.paint("a"), bold.paint("dd"));

    println!("{}{} - List existing items in your todo list", 
        bold_underline.paint("l"), bold.paint("ist"));

    println!("{}{} - Set an item as complete in your todo list", 
        bold_underline.paint("c"), bold.paint("omplete"));

    println!("{}{} - Removes ALL your completed items from the list", 
        bold_underline.paint("r"), bold.paint("emove"));

    println!("{}{} - Extra options", 
        bold_underline.paint("o"), bold.paint("ptions"));

    println!("{}{} - Exit the program", 
        bold_underline.paint("q"), bold.paint("uit"));
}
