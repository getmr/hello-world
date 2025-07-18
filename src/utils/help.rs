// use crate::libs; // Importing the libs module to access the add function

pub fn add_numbers(a: i32, b: i32) -> i32 {
    crate::libs::add(a, b) // Using the add function from libs module
}