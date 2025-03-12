// Title: Fibonacci
// Write a Rust function that takes an integer n and returns the n-th Fibonacci number using recursion.

#![allow(unused)]
use std::io;
impl fibonacci {
    pub fn fibonacci_recurssive(n:i64) -> i64 {
        if n<0 {
            panic!("{} is a negative number.", n);
        }
        match n {
            0 => 0, 
            1 | 2 => 1,
            _ => fibonacci(n-1) + fibonacci(n-2)
        }
    }

    pub fn ChoosenMethod(&self,&Choice : String){
        let recurssive = "recurssive";
        
    }
}

pub struct fibonacci {
    NumInLetter : String,
    MethodOfCalculation : bool,
    n : u64,
    fib : u64,
}


fn main() {
    let mut myfibonacci : fibonacci;
    io::stdin().read_line(&MyStruct::n : u64).except("Failed to read line.")

    println!("Hello, world!");
    println!("F(100) = {}",fibonacci(40));
}
