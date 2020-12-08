// To build just this module
// cargo build --bin hello
// Built exe file will be at target/debug/hello.exe
// To run hello module
// cargo run --bin hello

// extern crate hallo;
// use hallo::{f_sayhallo2, mal};

pub fn f_sayhello2(){
    println!("f_sayhello2");
}

// Test function with return value
pub fn mul(x: i32, y: &str) -> i32 {
    let _y = y.parse::<i32>().unwrap();
    x * _y
}

fn main(){
    f_sayhello2();
    // f_sayhallo2();
}