use anyhow::bail;
use std::fs::{self, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

pub fn load_todo_list() -> Result<Vec<String>, anyhow::Error> {
    let todo_list = if Path::new("todo_list.txt").exists() {
        fs::read_to_string("todo_list.txt")?
    } else {
        String::new()
    };
    let todo_list_lines: Vec<String> = todo_list.lines().map(|x| x.to_string()).collect();

    Ok(todo_list_lines)
}

pub fn run(
    config: &str,
    input: Vec<&str>,
    mut todo_list_lines: Vec<String>,
) -> Result<(), anyhow::Error> {
    if config == "view" {
        if todo_list_lines.len() == 0 {
            println!("-- empty --");
        }
        for (i, line) in todo_list_lines.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }
    } else if config == "add" {
        if input.len() == 0 {
            bail!("Need at least one item to add to todo list.")
        }
        let mut item_content = String::new();
        for text in input {
            item_content.push_str(text);
            item_content.push_str(" ");
        }
        todo_list_lines.push(item_content);
        overwrite_todo_list_file(&todo_list_lines)?;
    } else if config == "delete" {
        if input.len() == 0 {
            bail!("Need the item number of the item that is supposed to be removed.");
        }
        let input_key = *(input.get(0).unwrap());
        let key_number = input_key.parse::<usize>()?;
        if key_number - 1 >= todo_list_lines.len() {
            bail!("Item number of item to remove is too big.");
        }
        todo_list_lines.remove(key_number - 1);
        overwrite_todo_list_file(&todo_list_lines)?;
    }

    Ok(())
}

fn overwrite_todo_list_file(todo_list_lines: &Vec<String>) -> Result<(), anyhow::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("todo_list.txt")?;
    for line in todo_list_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
