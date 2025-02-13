# Rust Object Orientation Program
## Introduction
This project implements Object-Oriented Programming (OOP) in Rust, featuring two main components:

1. Binary Search Tree (BST) - A tree structure supporting insertion and searching operations.

2. Minesweeper  - A terminal-based Minesweeper game where players can reveal cells, flag mines, and win by uncovering all non-mine cells.

## Key Concepts
### **Encapsulation**
- Implemented using `struct`, `impl`, and `pub`.

- BST and Minesweeper structs encapsulate data, ensuring controlled access.

- Example: TreeNode encapsulates left and right children while exposing controlled methods.

### **Safe Memory Management with `Option<Box<T>>`**
- Rust avoids null pointers by using `Option<Box<T>>` instead of raw pointers.

- `None` represents the absence of a value, ensuring safe tree navigation.

- Example in BST:
```rust
struct TreeNode {
    value: i32,
    left: Option<Box<TreeNode>>, 
    right: Option<Box<TreeNode>>,
}
```
- This prevents dangling pointers and segmentation faults, enforcing memory safety at compile-time.