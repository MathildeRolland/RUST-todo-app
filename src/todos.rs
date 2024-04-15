
pub mod todos {
    #[derive(Debug)]
    pub struct Todo {
        pub id: u32,
        pub label: String,
        pub completed: bool
    }

    pub struct TodosController {
        pub todos: Vec<Todo>
    }

    impl TodosController {
        pub fn new() -> Self {
            let mut todos: Vec<Todo> = Vec::new();
            
            let todo1 = Todo {id: 1, label: String::from("Faire 100 pompes"), completed: false};
            let todo2 = Todo {id: 2, label: String::from("Lire un livre"), completed: false};
            let todo3 = Todo {id: 3, label: String::from("Postuler à 2 entreprises"), completed: false};
            
            
            todos.push(todo1);
            todos.push(todo2);
            todos.push(todo3);
            
            
            Self {
                todos,
            }
        }
   
        pub fn add_todo(todos: &mut Vec<Todo>, input: String) {
            let new_todo = Todo {id: 4, label: input, completed: false};
            todos.push(new_todo);
        }

        pub fn view_todos(todos: &[Todo]) {
            let todos_iter = todos.into_iter();
            for todo in todos_iter {
                println!("{}. [{}] {}", todo.id, if todo.completed {"x"} else {""}, todo.label);
            }
        } 
    }

}



