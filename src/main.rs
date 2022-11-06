// Local imports
use todo_cli::{
    db_handlers, 
    cli_args,
    menu
};

// Argument parser
use clap::Parser;

// import Exitting
use std::process::exit;

fn main() {
    let args = cli_args::CliArgs::parse();
    let mut connection = db_handlers::establish_connection();

    
    // Argument checks - exit after command is completed
    match args.add {
        Some(msg) => {  // Add new todo item
            db_handlers::create_todo_item(&mut connection, msg);
            exit(0);
        },
        None => {}, // Argument empty, do nothing
    }

    match args.list {
        true => {   // Argument used, true
            menu::list_todo(&mut connection);
            exit(0);
        },
        false => {}, // Argument is default (false) do nothing
    }

    match args.remove {
        true => {
            db_handlers::remove_completed(&mut connection);
            exit(0);
        },
        false => {},
    }

    match args.complete {
        Some(i) => { // Set item complete
            db_handlers::set_todo_complete(&mut connection, i);
            exit(0);
        },
        None => {}, // Argument empty, do nothing
    }


    // No other arguments used, print the 'default' menu
    menu::normal_menu(&mut connection);

}