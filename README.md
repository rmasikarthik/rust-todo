# Rust Todo List

A simple command-line Todo List application built with **Rust**. This project is designed as a beginner-friendly way to practice Rust fundamentals such as vectors, functions, loops, pattern matching, user input, and mutable references.

## Features

* Add new tasks
* View all tasks
* Mark tasks as completed
* Delete tasks
* Interactive terminal menu
* Input validation for task numbers

## Technologies Used

* **Rust**
* Standard Rust library
* Cargo

## Project Structure

```text
todo-list/
├── Cargo.toml
└── src/
    └── main.rs
```

## Prerequisites

Install Rust and Cargo on your system.

Verify the installation:

```bash
rustc --version
cargo --version
```

## Installation

Clone the repository:

```bash
git clone <your-repository-url>
```

Navigate to the project directory:

```bash
cd todo-list
```

## Run the Application

Use Cargo to run the application:

```bash
cargo run
```

You will see the following menu:

```text
===== TODO LIST =====
1. Add task
2. View tasks
3. Complete task
4. Delete task
5. Exit

Enter your choice:
```

## Example

### Add a Task

```text
Enter your choice: 1
Enter your task: Learn Rust
Task added successfully!
```

### Add Another Task

```text
Enter your choice: 1
Enter your task: Practice Linux
Task added successfully!
```

### View Tasks

```text
Enter your choice: 2

Your Tasks:
1. Learn Rust
2. Practice Linux
```

### Complete a Task

```text
Enter your choice: 3
Enter task number to complete: 1
Task completed!
```

The completed task will appear as:

```text
1. Learn Rust [COMPLETED]
2. Practice Linux
```

### Delete a Task

```text
Enter your choice: 4
Enter task number to delete: 2
Task deleted!
```

## Rust Concepts Practiced

This project covers several fundamental Rust concepts:

* Variables and mutable variables
* `Vec<String>`
* Functions
* `loop`
* `match`
* `if` statements
* User input with `std::io`
* String handling
* References
* Mutable references
* Iterators
* `enumerate()`
* Error handling with `Result`
* Parsing user input

## Limitations

Currently, tasks are stored only in memory. When the application exits, all tasks are lost.

Future versions could add:

* Persistent storage
* Save tasks to a JSON or text file
* Task priorities
* Due dates
* Search functionality
* Task categories
* Better completed-task handling
* Unit tests
* A more advanced terminal UI

## Learning Goal

The goal of this project is to build a small but practical Rust application while learning the fundamentals of Rust programming and command-line application development.
