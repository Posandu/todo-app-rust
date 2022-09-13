use crate::todo::*;
use crate::utils;
use colorize::*;

pub fn start() {
    utils::init();

    let args = utils::get_args();

    match args.command.as_str() {
        "a" => add(args.arguments),
        "l" => list(),
        "d" => done(args.arguments),
        "r" => remove(args.arguments),
        "q" => std::process::exit(0),
        _ => {
            println!("{}", "            No command found - Showing help".black());

            let help = format!(
                "
            {} {}
            {}
            -----
        
            Help:
        
            Command   | Arguments | Description
            {}           text        Add a new todo
            {}                       List all todos
            {}           id          Mark a todo as done
            {}           id          Delete a todo
        ",
                "Welcome to".grey(),
                "TodoBook".cyan(),
                "Simple todo app written in Rust".black(),
                "a".cyan(),
                "l".blue(),
                "d".green(),
                "r".red()
            );

            println!("{help}");
        }
    }
}
