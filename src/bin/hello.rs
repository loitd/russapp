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

// Linux only
#[cfg(target_os = "linux")]
fn linux_only() {
    println!("On linux only");
}

// check linux inside
fn checky_os(){
    if cfg!(target_os = "linux") {
        println!("Yay, you are on linux");
    } else if cfg!(target_os = "windows") {
        println!("Yay, you are on windows");
    } else {
        println!("Yay, os not supported!");
    }
}

fn main(){
    f_sayhello2();
    checky_os();
}

//-----------------------------TESTS
#[cfg(test)]
mod tests {

    #[test]
    fn test1(){
        assert_eq!(2+2, 2*2);
    }
}









