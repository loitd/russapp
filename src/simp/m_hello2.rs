pub fn f_sayhello2(){
    println!("f_sayhello2");
}

// Test function with return value
pub fn mul(x: i32, y: &str) -> i32 {
    let _y = y.parse::<i32>().unwrap();
    x * _y
}