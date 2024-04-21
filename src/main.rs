pub mod todos;
pub mod utils;

use todos::TodosController;
use utils::{read_user_input, parse_user_input};


fn ask_user_desired_action_on_todo() ->u32 {
    println!("1. Marquer comme Fait/A faire");
    println!("2. Editer");
    println!("3. Supprimer");

    let user_input_as_number = parse_user_input(read_user_input());
    user_input_as_number
}

fn handle_todo_choice(todos_controller: &mut TodosController, chosen_todo_index: u32) {
    if chosen_todo_index as usize > todos_controller.todos.len() {
        println!("Cette todo n'existe pas");
        return;
    }
    
    let selected_action = ask_user_desired_action_on_todo();
    match selected_action {
        1 => todos_controller.toggle_todo_completion(chosen_todo_index),
        2 => {
            let todo_label_input = read_user_input();
            todos_controller.edit_todo(chosen_todo_index, todo_label_input)
        },
        3 => todos_controller.delete_todo(chosen_todo_index),
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
                    handle_todo_choice(&mut todos_controller, chosen_todo_index);
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
