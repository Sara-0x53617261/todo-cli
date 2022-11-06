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
    // Get startup arguments
    let args = cli_args::CliArgs::parse();

    // Setup db connection
    let mut connection = db_handlers::establish_connection();

    
    // Check our arguments and execute where needed
    // startup argument commands exit after completion

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


    // No other arguments used, go to the default application loop
    menu::normal_menu(&mut connection);

}