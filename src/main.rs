use clap::{ArgGroup, Parser};
use std::io::Write;
use std::{io, process::exit};
use ansi_term;

// database imports
use diesel::{sqlite::SqliteConnection, Connection, QueryDsl};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

// local imports
use todo_cli::models::*;
use todo_cli::schema::*;
use todo_cli::create_todo_item;
use todo_cli::set_todo_complete;

// Main menu art
use todo_cli::ascii::ART;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("group")
        .required(false)
        .args(&["add", "list", "complete"])
))]
struct Args {

    #[arg(short, long, required = false )]
    add: Option<String>,    // Takes a message to add to the todo list

    #[arg(short, long, default_value_t = false, required = false)]
    list: bool,

    #[arg(short, long, required = false)]
    complete: Option<i32>,  // Takes ID of the item to set as completed
}


fn main() {
    let args = Args::parse();

    let connection = &mut establish_connection();

    // Add argument used
    if args.add.as_deref() != None {
        let new_message = args.add.unwrap();
        println!("Directly adding new item with value: {}", &new_message);

        create_todo_item(connection, new_message);
    }
    // List argument given
    else if args.list {
        list(connection);
    }
    // Complete argument used
    else if args.complete != None {
        let message_id = args.complete.unwrap();
        set_todo_complete(connection, message_id);
    }
    // No arguments given, go to default menu
    else {
        normal_menu(connection);
    }

}

/*
    - No arguments provided - default menu stuff
*/
fn normal_menu(connection: &mut SqliteConnection) {
    /*
    Menu options;
    add, list, complete, options, quit
    */

    // Text styles
    let bold = ansi_term::Style::new().bold();
    let bold_underline = ansi_term::Style::new().bold().underline();

    // Prints the logo
    println!("{}", ART);

    loop {
        // input buffer
        let mut line = String::new();

        // Menu options
        println!("\n{}", bold_underline.paint("Menu options:"));
        println!("{}{} - Add a new item to your todo list", 
            bold_underline.paint("a"), bold.paint("dd"));

        println!("{}{} - List existing items in your todo list", 
            bold_underline.paint("l"), bold.paint("ist"));

        println!("{}{} - Set an item as complete in your todo list", 
            bold_underline.paint("c"), bold.paint("omplete"));

        println!("{}{} - Extra options", 
            bold_underline.paint("o"), bold.paint("ptions"));

        println!("{}{} - Exit the program", 
            bold_underline.paint("q"), bold.paint("uit"));

        // Get input
        print!("$> ");
        _ = io::stdout().flush();

        io::stdin().read_line(&mut line).unwrap();

        match line.trim() {
            "a" | "add" => add(connection),
            "l" | "list" => list(connection),
            "c" | "complete" => new_complete(connection),
            "o" | "options" => options(),
            "q" | "quit" => exit(0),
            _ => println!("Unknown command!")
        }

    }

}

fn add(conn: &mut SqliteConnection) {
    // Get new todo item
    let mut new_message = String::new();

    println!("\nPlease enter what you want your new todo item to say");

    io::stdin().read_line(&mut new_message).unwrap();

    //let new_id = db_query::add_new(new_message);

    //if !new_id.is_none() {
    //    println!("Successfully added, your new item has the ID: {}\nreturning to menu", new_id.unwrap());
    //}
}

fn list(connection: &mut SqliteConnection) {
    // Get and print todo list
    let results = todo_items::table
        .limit(20)
        .load::<TodoItem>(connection)
        .expect("Error loading items");

    println!("You have: {} items, in your todo list.", results.len());

    for item in results {
        print!("Item ID: {} | ", item.id);

        let mut style = ansi_term::Style::new();

        if item.completed {
            style = ansi_term::Style::new().strikethrough();
        }

        println!("Message: {}", style.paint(item.message));
    }
}

fn new_complete(conn: &mut SqliteConnection) {
    // gets id of item to set as complete
}

fn options() {
    // Options - todo
}


/* DATABASE SETUP  */
fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("No database url");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(
            |_| panic!("Error connecting to database {}", database_url)
        )
}

/*
usages;

todo add new todo item
Adds new item to the todo list

todo list
Lists all items in the todo list

todo complete 6
sets item 6 as completed - strikethrough the text in todo list
also prints todo list but updated

todo .
no arguments given just launch normally 
and give the above options inside the program

----------------------------------------------------------------

TodoStruct {
    ID, MESSAGE, COMPLETED
}
*/
