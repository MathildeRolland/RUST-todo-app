pub mod todos;
pub mod utils;

use todos::{Todo, TodosController};
use utils::{read_user_input, parse_user_input};

fn get_selected_todo(todos: &mut Vec<Todo>, chosen_todo_index: u32) -> Option<&mut Todo> {
    let selected_todo = todos.into_iter().find(|todo| {
        todo.id == chosen_todo_index
    });
    selected_todo
}

fn ask_user_desired_action() ->u32 {
    println!("1. Marquer comme Fait/A faire");
    println!("2. Editer");
    println!("3. Supprimer");

    let user_input_as_number = parse_user_input(read_user_input());
    user_input_as_number
}

fn handle_todo_choice(todos: &mut Vec<Todo>, chosen_todo_index: u32) {
    if chosen_todo_index as usize > todos.len() {
        println!("Cette todo n'existe pas");
        return;
    }
    
    let selected_action = ask_user_desired_action();
    match selected_action {
        1 => {
            let selected_todo_option = get_selected_todo(todos, chosen_todo_index);
            if selected_todo_option.is_some() {
                let todo = &mut todos[chosen_todo_index as usize - 1];
                todo.completed = !todo.completed;
                println!("{:?}", todos);
            }
        },
        2 => {},
        3 => {},
        _ => println!("Veuillez sélectionner une action valide")
     }
    

}

fn main() {
    let mut todos_controller = TodosController::new();

    loop {
        println!("Que souhaitez-vous faire?");
        println!("1. Voir la liste des todos");
        println!("2. Ajouter un todo");

        let user_input = parse_user_input(read_user_input());
    
        match user_input {
            1 => {
                loop {
                    todos_controller.view_todos();
                    let chosen_todo_index = parse_user_input(read_user_input());
                    handle_todo_choice(&mut todos_controller.todos, chosen_todo_index);
                }
            },
            2 => {
                println!("Veuillez renseigner le titre de votre nouvelle todo:");
                let input = read_user_input();
                todos_controller.add_todo(input);
            },
            _ => {}
        }
    }


}
