pub mod todos;
use todos::todos::{Todo, TodosController};
use std::io;

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

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let user_input_as_number = user_input.trim().parse::<u32>().expect("Invalid input");
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
    
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
    
        let user_input_as_number = user_input.trim().parse::<u32>().expect("Invalid input");
    
        match user_input_as_number {
            1 => {
                loop {
                    todos_controller.view_todos();
                    let mut chosen_todo_index = String::new();
                    io::stdin().read_line(&mut chosen_todo_index).expect("Failed to read line");
                    let chosen_todo_index_as_number = chosen_todo_index.trim().parse::<u32>().expect("Invalid input");
                    handle_todo_choice(&mut todos_controller.todos, chosen_todo_index_as_number);
                }
            },
            2 => {
                println!("Veuillez renseigner le titre de votre nouvelle todo:");
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                todos_controller.add_todo(input);
            },
            _ => {}
        }
    }


}
