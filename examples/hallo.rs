// To build just this module. To configure crate-type for example in TOML.
// cargo build --example hallo
// Built the example file will be at target/debug/examples/
// By default, examples are executable binaries (with a main() function). 
// You can specify the crate-type field to make an example be compiled as a library

pub fn f_sayhallo2(){
    println!("f_sayhallo2");
}

// Test function with return value
pub fn mal(x: i32, y: &str) -> i32 {
    let _y = y.parse::<i32>().unwrap();
    x * _y
}