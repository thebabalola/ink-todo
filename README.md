# Todo Contract - ink! Smart Contract

A simple todo management smart contract built with ink! for Substrate blockchains.

## Features

- ✅ Create, read, update, and delete todos
- ✅ Toggle completion status
- ✅ Event emission for off-chain monitoring
- ✅ Demonstrates key ink! attributes

## Contract Functions

| Function | Description | Returns |
|----------|-------------|---------|
| `create_todo(title, description)` | Create a new todo | `u32` (todo ID) |
| `get_todo(id)` | Get a specific todo | `Option<Todo>` |
| `get_all_todos()` | Get all todos | `Vec<Todo>` |
| `update_todo(id, title, description)` | Update todo content | `bool` |
| `toggle_todo(id)` | Toggle completion status | `bool` |
| `delete_todo(id)` | Delete a todo | `bool` |
| `get_todo_count()` | Get total todo count | `u32` |

## Todo Structure

```rust
struct Todo {
    id: u32,
    title: String,
    description: String,
    completed: bool,
}
```

## Getting Started

### Build
```bash
cargo build
```

### Test
```bash
cargo test
```

### Deploy
```bash
cargo contract build
```

## Usage Example

```rust
// Create a todo
let todo_id = contract.create_todo("Learn ink!".to_string(), "Build my first contract".to_string());

// Get the todo
let todo = contract.get_todo(todo_id);

// Toggle completion
contract.toggle_todo(todo_id);

// Update todo
contract.update_todo(todo_id, Some("Updated title".to_string()), None);

// Delete todo
contract.delete_todo(todo_id);
```

## Dependencies

- `ink = "5.1.1"` - ink! smart contract framework
- `scale` - SCALE codec for data encoding
- `scale-info` - Type information for metadata

---

**Note:** This is a simple educational contract for learning ink! development.
