use crate::db::core::handle_input;
use crate::db::db::TodoList;
use crate::todos::todos::Todo;
use std::io::{self, Write};
use std::process;

#[derive(Debug)]
enum InputType {
    Number(usize),
    Text(String),
}

fn print_static_options(todo_app: &TodoList) {
    println!("1. Add Todo");
    println!("2. Edit Todo");
    println!("3. Delete Todo");
    println!("4. Mark Todo as completed");
    println!("5. Exit");

    println!("{:#?}", todo_app);

    println!("Sl Tasks                             Status        Created At         Updated At");

    todo_app.todos.iter().enumerate().for_each(|(index, todo)| {
        if !todo.is_deleted {
            println!(
                "{}. {} {} {} {}",
                index,
                todo.task,
                if todo.is_completed {
                    "Completed"
                } else {
                    "Not Completed"
                },
                todo.create_at,
                todo.update_at
            );
        }
    });
}

pub fn print_options() -> () {
    let mut todo_db = TodoList::new();
    print_static_options(&todo_db);
    loop {
        print!("Enter: ");

        io::stdout().flush().expect("Failed to flush");

        let input = match handle_input() {
            Ok(value) => value.trim().to_string(),
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        let value = parse_input(input);

        match value {
            InputType::Number(value) => {
                handle_options(value, &mut todo_db);
            }
            _ => {
                println!("Invalid Option");
            }
        }
    }
}

fn parse_input(input: String) -> InputType {
    if let Ok(num) = input.parse::<usize>() {
        InputType::Number(num)
    } else {
        InputType::Text(input)
    }
}

pub fn handle_options(option: usize, todo_app: &mut TodoList) {
    match option {
        option if option == 1 => {
            handle_add_todo(todo_app);
            print_static_options(&todo_app);
        }
        option if option == 2 => {
           handle_update_todo(todo_app);
        }
        option if option == 3 => {
            handle_delete_todo(todo_app);
        }
        option if option == 4 => {
            handle_complete_todo(todo_app)
        }
        option if option == 5 => {
            println!("Bye!");
            process::exit(1)
        }
        _ => {
            println!("Invalid Option");
        }
    }
}

fn handle_add_todo(todo_app: &mut TodoList) {
    print!("Enter Task: ");
    let input = match handle_input() {
        Ok(value) => value.trim().to_string(),
        _ => {
            println!("Invalid input");
            return;
        }
    };

    let todo = Todo::new(input);

    todo_app.todos.push(todo);

    todo_app.write();

    println!("{:#?}", todo_app);
}

fn handle_delete_todo(todo_app: &mut TodoList) {
    print!("Are you sure to delete the task? (yes/no)");
    io::stdout().flush().expect("Failed to flush!");
    let input = match handle_input() {
        Ok(value) => value.trim().to_string(),
        _ => {
            println!("Invalid input");
            return;
        }
    };

    match input.as_str() {
        "yes" | "y" => {
            print!("Enter the Sl no of the Task: ");
            io::stdout().flush().expect("Failed to flush!");
            let id = match handle_input() {
                Ok(value) => value.trim().to_string(),
                _ => {
                    println!("Invalid input");
                    return;
                }
            };

            let index = match parse_input(id) {
                InputType::Number(v) => v,
                InputType::Text(v) => {
                    println!("Invalid Id: {}", v);
                    return;
                }
            };

            let mut_index = index;

            todo_app.remove(index);
            todo_app.write();

            println!("Todo Deleted in the Id: {}", mut_index);

            print_options();
        }
        "no" | "n" => {
            println!("Thanks for the confirmation. I will redirect to Options!");
            print_options();
        }
        _ => {
            println!("Invalid Option. Please retry again    !");
            handle_delete_todo(todo_app);
        }
    }
}

fn handle_update_todo(todo_app: &mut TodoList) {
    print!("Enter the id of the todo?");
    io::stdout().flush().expect("Failed to flush!");
    let id = match handle_input() {
        Ok(value) => value.trim().to_string(),
        _ => {
            println!("Invalid input");
            return;
        }
    };
    let index = match parse_input(id) {
        InputType::Number(v) => v,
        InputType::Text(v) => {
            println!("Invalid Id: {}", v);
            return;
        }
    };
    print!("Enter the task to update?");
    let task = match handle_input() {
        Ok(value) => value.trim().to_string(),
        _ => {
            println!("Invalid input");
            return;
        }
    };

    todo_app.update_task(index, task);
}


fn handle_complete_todo(todo_app: &mut TodoList) {
    print!("Are you sure to mark as completed the task? (yes/no)");
    io::stdout().flush().expect("Failed to flush!");
    let input = match handle_input() {
        Ok(value) => value.trim().to_string(),
        _ => {
            println!("Invalid input");
            return;
        }
    };

    match input.as_str() {
        "yes" | "y" => {
            print!("Enter the Sl no of the Task: ");
            io::stdout().flush().expect("Failed to flush!");
            let id = match handle_input() {
                Ok(value) => value.trim().to_string(),
                _ => {
                    println!("Invalid input");
                    return;
                }
            };

            let index = match parse_input(id) {
                InputType::Number(v) => v,
                InputType::Text(v) => {
                    println!("Invalid Id: {}", v);
                    return;
                }
            };

            let mut_index = index;

            todo_app.handle_mark_as_completed(index);
            todo_app.write();

            println!("Todo marked as completed in the Id: {}", mut_index);

            print_options();
        }
        "no" | "n" => {
            println!("Thanks for the confirmation. I will redirect to Options!");
            print_options();
        }
        _ => {
            println!("Invalid Option. Please retry again    !");
            handle_delete_todo(todo_app);
        }
    }
}