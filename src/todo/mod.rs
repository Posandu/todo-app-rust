use crate::structs::Todo;
use crate::utils;
use colorize::*;

///
/// Add todo
///
pub fn add(title: String) {
    if title.len() < 1 {
        println!("{}", "No title provided".red());

        return;
    }

    let mut todos = utils::get_todos().unwrap();

    let todo = Todo {
        created_at: utils::get_timestamp(),
        title,
        done: false,
        id: utils::get_id(),
        updated_at: utils::get_timestamp(),
    };

    todos.push(todo);

    utils::save_todos(todos);

    println!("{}", "Added todo".green());
}

///
/// List todos
/// 
pub fn list() {
    let todos = utils::get_todos().unwrap();

    if todos.len() == 0 {
        println!("{}", "No todos".red());
        return;
    }

    println!(
        "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
        "ID", "Title", "Created at", "Updated at", "Done"
    );

    println!();
    
    for todo in todos {
        println!(
            "{0: <5} | {1: <20} | {2: <20} | {3: <20} | {4: <20}",
            todo.id,
            todo.title,
            todo.created_at,
            todo.updated_at,
            if todo.done { "Completed 😸".green() } else { "No 😿".red() }
        );
    }
}

///
/// Mark todo as done
/// 
pub fn done(id: String) {
    let mut todos = utils::get_todos().unwrap();
    let id = id.parse::<u32>().unwrap_or(0);

    let exists = todos.iter().any(|todo| todo.id == id);

    if !exists {
        println!("{}", "Todo not found".red());
        return;
    }

    for todo in &mut todos {
        if todo.id == id {
            todo.done = true;
            todo.updated_at = utils::get_timestamp();
        }
    }

    utils::save_todos(todos);

    println!("{}", "Marked todo as done".green());
}

///
/// Remove todo
/// 
pub fn remove(id: String) {
    let mut todos = utils::get_todos().unwrap();
    let id = id.parse::<u32>().unwrap_or(0);

    let exists = todos.iter().any(|todo| todo.id == id);

    if !exists {
        println!("{}", "Todo not found".red());
        return;
    }

    todos.retain(|todo| todo.id != id);

    utils::save_todos(todos);

    println!("{}", "Removed todo".green());
}
