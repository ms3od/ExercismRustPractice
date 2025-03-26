// Title: Fibonacci
// Write a Rust function that takes an integer n and returns the n-th Fibonacci number using recursion.

#![allow(unused)]
use std::io;
impl fibonacci {
    pub fn fibonacci_recurssive(n:i64) -> i64 {
        if n<0 {
            panic!("{n} is a negative number.");
        }
        match n {
            0 => 0, 
            1 | 2 => 1,
            _ => fibonacci::fibonacci_recurssive(n-1) + fibonacci::fibonacci_recurssive(n-2)
        }
    }

    pub fn ChoosenMethod(&self,&choice : String) {
        let recurssive = "recurssive";
        
    }
}

pub struct fibonacci {
    num_in_letter : String,
    method_of_calculation : bool,
    n : u64,
    fib : u64,
}


fn main() {
    let mut my_fibonacci : fibonacci;
    io::stdin().read_line(my_fibonacci::n).except("Failed to read line.");

    println!("Hello, world!");
    println!("F(100) = {}",my_fibonacci::fibonacci_recurssive(40));
}
