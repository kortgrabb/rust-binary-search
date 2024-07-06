//! # Binary Search Implementation in Rust
//! 
//! This project demonstrates a generic binary search algorithm implemented in Rust.
//! It uses the use of generics, the `Ord` trait, allowing any ordered type to be used, and Rust's pattern matching.

use std::cmp::Ordering;

/// Performs a binary search on a sorted slice.
///
/// # Arguments
///
/// * `target` - The value to search for.
/// * `arr` - A sorted slice of values to search in.
///
/// # Returns
///
/// * `Some(usize)` - The index of the target if found.
/// * `None` - If the target is not in the array.
///
/// # Examples
///
/// ```
/// let arr = [1, 3, 4, 6, 8, 9, 11];
/// assert_eq!(binary_search(&6, &arr), Some(3));
/// assert_eq!(binary_search(&5, &arr), None);
/// ```
fn binary_search<T: Ord>(target: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        match target.cmp(&arr[mid]) {
            Ordering::Less => right = mid,
            Ordering::Greater => left = mid + 1,
            Ordering::Equal => return Some(mid),
        }
    }

    None
}

fn main() {
    // Example with integers
    let int_arr = [0, 4, 7, 9, 23, 57, 79, 89];
    let int_target = 23;
    match binary_search(&int_target, &int_arr) {
        Some(index) => println!("Integer {} found at index {}.", int_target, index),
        None => println!("Integer {} not found.", int_target),
    }

    // Example with strings
    let string_arr = ["apple", "banana", "cherry", "apple", "raspberry"];
    let string_target = "raspberry";
    match binary_search(&string_target, &string_arr) {
        Some(index) => println!("String '{}' found at index {}.", string_target, index),
        None => println!("String '{}' not found.", string_target),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_integers() {
        let arr = [1, 3, 4, 6, 8, 9, 11];
        assert_eq!(binary_search(&6, &arr), Some(3));
        assert_eq!(binary_search(&1, &arr), Some(0));
        assert_eq!(binary_search(&11, &arr), Some(6));
        assert_eq!(binary_search(&7, &arr), None);
        assert_eq!(binary_search(&0, &arr), None);
        assert_eq!(binary_search(&12, &arr), None);
    }

    #[test]
    fn test_binary_search_strings() {
        let arr = ["apple", "banana", "cherry", "apple", "raspberry"];
        assert_eq!(binary_search(&"cherry", &arr), Some(2));
        assert_eq!(binary_search(&"apple", &arr), Some(0));
        assert_eq!(binary_search(&"raspberry", &arr), Some(4));
        assert_eq!(binary_search(&"blueberry", &arr), None);
        assert_eq!(binary_search(&"jacobfet", &arr), None);
        assert_eq!(binary_search(&"zebra", &arr), None);
    }
}