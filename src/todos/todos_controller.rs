use crate::todos::todo_model::Todo;
use crate::todos::todos_service::TodosService;


pub struct TodosController {
    todos_service: TodosService
}

impl TodosController {
    pub fn new(todos_service: TodosService) -> TodosController {
        Self {
            todos_service
        }
    }

    pub fn add_todo(&mut self, input: String) {
        self.todos_service.add_todo(input);
    }

    pub fn delete_todo(&mut self, todo_index: u32) {
        self.todos_service.delete_todo(todo_index);
    }

    pub fn edit_todo(&mut self, todo_index: u32, new_label: String) {
        self.todos_service.edit_todo(todo_index, new_label);
    }

    pub fn view_todos(&self) {
        self.todos_service.view_todos();
    }

    pub fn get_todos(&self) -> &Vec<Todo> {
        self.todos_service.get_todos()
    }

    pub fn toggle_todo_completion(&mut self, todo_index: u32) {
        self.todos_service.toggle_todo_completion(todo_index);
    }
}