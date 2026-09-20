fn main() {
    let mut todos: Vec<String> = Vec:new();

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
            "2" => view_task(&mut todos),
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
