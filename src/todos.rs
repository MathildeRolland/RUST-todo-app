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
        for todo in &self.todos {
            println!("{}. [{}] {}", todo.id, if todo.completed {"x"} else {""}, todo.label);
        }
    } 

    pub fn toggle_todo_completion(&mut self, todo_index: u32) {
        let selected_todo_option = self.get_todo_by_id(todo_index);
        if selected_todo_option.is_some() {
            let todo = &mut self.todos[todo_index as usize - 1];
            todo.completed = !todo.completed;
        }
    }

    pub fn get_todo_by_id(&self, todo_index: u32) -> Option<&Todo> {
        let selected_todo = self.todos.iter().find(|todo| {
            todo.id == todo_index
        });
        selected_todo
    }
}





