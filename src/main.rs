use clap::{App, Arg};
use std::process;

mod lib;

fn main() {
    let matches = App::new("TODO List")
        .author("MFRIbrahim")
        .about(
"This tool is a simple todo list. You can add an item by simply entering 'add' followed by whatever
you want to add to the todo list. You can delete an item by entering 'delete' followed by the item
number of the item that you want to delete and you can view the whole todo list by entering
'view'.")
        .arg(
            Arg::with_name("INPUT")
                .help(
"The operation that the user wants to perform. If the operation is 'add' then the following
arguments will be used to add an item to the todo list. In case of the 'delete' operation, one
additional argument for the item number of the item that is supposed to be removed is required.")
                .required(true)
                .multiple(true),
        )
        .get_matches();

    let todo_list_lines = lib::load_todo_list().unwrap_or_else(|err| {
        eprintln!("Problem loading the todo list: {}", err);
        process::exit(1);
    });
    let mut input: Vec<&str> = matches.values_of("INPUT").unwrap().collect();
    let config = match input.remove(0) {
        value if value == "view" || value == "add" || value == "delete" => value,
        _ => {
            eprintln!("OPERATION needs to be 'view', 'add' or 'delete'");
            process::exit(1);
        }
    };

    lib::run(config, input, todo_list_lines).unwrap_or_else(|err| {
        eprintln!("Problem updating the todo list: {}", err);
        process::exit(1);
    });
}
