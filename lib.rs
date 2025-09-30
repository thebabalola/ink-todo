#![cfg_attr(not(feature = "std"), no_std, no_main)]

use ink::prelude::vec::Vec;
use ink::storage::Mapping;

#[ink::contract]
mod todo_contract {

    use super::*;

    /// A simple Todo item
    #[derive(scale::Decode, scale::Encode, Clone, PartialEq, Debug)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    #[derive(ink::storage::traits::StorageLayout)]
    pub struct Todo {
        pub id: u32,
        pub title: String,
        pub description: String,
        pub completed: bool,
    }

    /// Events emitted by the contract
    #[ink(event)]
    pub struct TodoCreated {
        #[ink(topic)]
        pub id: u32,
        #[ink(topic)]
        pub title: String,
    }

    #[ink(event)]
    pub struct TodoUpdated {
        #[ink(topic)]
        pub id: u32,
        #[ink(topic)]
        pub completed: bool,
    }

    #[ink(event)]
    pub struct TodoDeleted {
        #[ink(topic)]
        pub id: u32,
    }

    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct TodoContract {
        /// Mapping from todo ID to Todo item
        todos: Mapping<u32, Todo>,
        /// Counter for generating unique todo IDs
        next_id: u32,
    }

    impl TodoContract {
        /// Constructor that initializes the contract
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                todos: Mapping::new(),
                next_id: 1,
            }
        }

        /// Default constructor
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new()
        }

        /// Create a new todo item
        #[ink(message)]
        pub fn create_todo(&mut self, title: String, description: String) -> u32 {
            let id = self.next_id;
            let todo = Todo {
                id,
                title: title.clone(),
                description,
                completed: false,
            };
            
            self.todos.insert(id, &todo);
            self.next_id += 1;

            self.env().emit_event(TodoCreated { id, title });
            id
        }

        /// Get a todo item by ID
        #[ink(message)]
        pub fn get_todo(&self, id: u32) -> Option<Todo> {
            self.todos.get(id)
        }

        /// Get all todo items
        #[ink(message)]
        pub fn get_all_todos(&self) -> Vec<Todo> {
            let mut all_todos = Vec::new();
            for i in 1..self.next_id {
                if let Some(todo) = self.todos.get(i) {
                    all_todos.push(todo);
                }
            }
            all_todos
        }

        /// Update a todo item
        #[ink(message)]
        pub fn update_todo(&mut self, id: u32, title: Option<String>, description: Option<String>) -> bool {
            if let Some(mut todo) = self.todos.get(id) {
                if let Some(new_title) = title {
                    todo.title = new_title;
                }
                if let Some(new_description) = description {
                    todo.description = new_description;
                }
                
                self.todos.insert(id, &todo);
                true
            } else {
                false
            }
        }

        /// Toggle todo completion status
        #[ink(message)]
        pub fn toggle_todo(&mut self, id: u32) -> bool {
            if let Some(mut todo) = self.todos.get(id) {
                todo.completed = !todo.completed;
                self.todos.insert(id, &todo);
                
                self.env().emit_event(TodoUpdated { 
                    id, 
                    completed: todo.completed 
                });
                true
            } else {
                false
            }
        }

        /// Delete a todo item
        #[ink(message)]
        pub fn delete_todo(&mut self, id: u32) -> bool {
            if self.todos.get(id).is_some() {
                self.todos.remove(id);
                self.env().emit_event(TodoDeleted { id });
                true
            } else {
                false
            }
        }

        /// Get the total number of todos
        #[ink(message)]
        pub fn get_todo_count(&self) -> u32 {
            self.next_id - 1
        }
    }

}
