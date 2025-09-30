#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod todo {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Task {
        pub id: u32,
        pub description: String,
        pub completed: bool,
    }

    #[ink(event)]
    pub struct TaskCreated {
        #[ink(topic)]
        pub id: u32,
        #[ink(topic)]
        pub description: String,
    }

    #[ink(event)]
    pub struct TaskCompleted {
        #[ink(topic)]
        pub id: u32,
    }

    #[ink(event)]
    pub struct TaskRemoved {
        #[ink(topic)]
        pub id: u32,
    }

    #[ink(event)]
    pub struct TaskUpdated {
        #[ink(topic)]
        pub id: u32,
        #[ink(topic)]
        pub description: String,
    }

    #[ink(storage)]
    pub struct Todo {
        tasks: Vec<Task>,
        next_id: u32,
    }

    impl Default for Todo {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Todo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                tasks: Vec::new(),
                next_id: 0,
            }
        }

        #[ink(message)]
        pub fn add_task(&mut self, description: String) -> u32 {
            let id = self.next_id;
            let task = Task {
                id,
                description: description.clone(),
                completed: false,
            };
            self.tasks.push(task);
            self.next_id = self.next_id.saturating_add(1);
            
            self.env().emit_event(TaskCreated {
                id,
                description,
            });
            
            id
        }

        #[ink(message)]
        pub fn complete_task(&mut self, id: u32) -> bool {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                task.completed = true;
                
                self.env().emit_event(TaskCompleted { id });
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn remove_task(&mut self, id: u32) -> bool {
            if let Some(pos) = self.tasks.iter().position(|t| t.id == id) {
                self.tasks.remove(pos);
                
                self.env().emit_event(TaskRemoved { id });
                true
            } else {
                false
            }
        }

        #[ink(message)]
        pub fn get_tasks(&self) -> Vec<Task> {
            self.tasks.clone()
        }

        #[ink(message)]
        pub fn get_task(&self, id: u32) -> Option<Task> {
            self.tasks
                .iter()
                .find(|t| t.id == id)
                .cloned()
        }

        #[ink(message)]
        pub fn update_task(&mut self, id: u32, description: String) -> bool {
            if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
                task.description = description.clone();
                
                self.env().emit_event(TaskUpdated {
                    id,
                    description,
                });
                true
            } else {
                false
            }
        }
    }
}