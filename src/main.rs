use std::io::{self, Write};

fn main() {
    let mut todos: Vec<String> = Vec::new();

    loop {
        println!("\n ----- TODO LIST ----- ");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Complete task");
        println!("4. Delete task");
        println!("5. Exit");

        println!("Enter your option :");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_task(&mut todos),
            "2" => view_tasks(&mut todos),
            "3" => complete_task(&mut todos),
            "4" => delete_task(&mut todos),
            "5" => {
                println!("Thank you!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}

fn add_task(todos: &mut Vec<String>) {
    print!("Enter your task: ");
    io::stdout().flush().unwrap();

    let mut task = String::new();
    io::stdin().read_line(&mut task).unwrap();

    todos.push(task.trim().to_string());

    println!("Task added successfully!");
}

fn view_tasks(todos: &Vec<String>) {
    if todos.is_empty() {
        println!("No tasks found.");
        return;
    }
    println!("\nYour Tasks:");

    for (index, task) in todos.iter().enumerate() {
        println!("{}. {}", index + 1, task);
    }
}

fn complete_task(todos: &mut Vec<String>) {
    view_tasks(todos);

    if todos.is_empty() {
        return;
    }

    print!("Enter task number to complete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if number >= 1 && number <= todos.len() {
        todos[number - 1].push_str(" [COMPLETED]");
        println!("Task completed!");
    } else {
        println!("Invalid task number.");
    }
}

fn delete_task(todos: &mut Vec<String>) {
    view_tasks(todos);

    if todos.is_empty() {
        return;
    }

    print!("Enter task number to delete: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let number: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    if number >= 1 && number <= todos.len() {
        todos.remove(number - 1);
        println!("Task deleted!");
    } else {
        println!("Invalid task number.");
    }
}
