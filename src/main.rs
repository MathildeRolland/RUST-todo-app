pub mod todos;
use todos::Todo;
use std::io;

fn init_todos() -> Vec<Todo> { 
    let mut todos: Vec<Todo> = Vec::new();

    let todo1 = Todo {id: 1, label: String::from("Faire 100 pompes"), completed: false};
    let todo2 = Todo {id: 2, label: String::from("Lire un livre"), completed: false};
    let todo3 = Todo {id: 3, label: String::from("Postuler à 2 entreprises"), completed: false};

    todos.push(todo1);
    todos.push(todo2);
    todos.push(todo3);

    todos
}

fn view_todos(todos: &[Todo]) {
    let todos_iter = todos.into_iter();
    for todo in todos_iter {
        println!("{}. {}", todo.id, todo.label);
    }
} 

fn main() {
    let todos = self::init_todos();

    println!("Que souhaitez-vous faire?");
    println!("1. Voir la liste des todos");
    println!("2. Ajouter un todo");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_input_as_number = user_input.trim().parse::<u32>().expect("Invalid input");

    match user_input_as_number {
        1 => {
            self::view_todos(&todos);
            let mut chosen_todo_index = String::new();
            io::stdin().read_line(&mut chosen_todo_index).expect("Failed to read line");
        },
        2 => {},
        _ => {}
    }

}
