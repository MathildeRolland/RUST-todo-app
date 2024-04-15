
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
            let todos = vec![
                Todo {id: 1, label: String::from("Faire 100 pompes"), completed: false},
                Todo {id: 2, label: String::from("Lire un livre"), completed: false},
                Todo {id: 3, label: String::from("Postuler à 2 entreprises"), completed: false},
            ];
            
            Self {
                todos,
            }
        }
   
        pub fn add_todo(&mut self, input: String) {
            let new_todo = Todo {id: 4, label: input, completed: false};
            self.todos.push(new_todo);
        }

        pub fn view_todos(&self) {
            let todos_iter = &self.todos;
            for todo in todos_iter {
                println!("{}. [{}] {}", todo.id, if todo.completed {"x"} else {""}, todo.label);
            }
        } 
    }

}



