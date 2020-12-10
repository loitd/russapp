/// First line is for short desc
///
/// Next lines for detailed documentation.
/// still detailed. Code blocks start with triple backquotes
///
/// ```
/// let result = russapp::m_hello3::f_hello3();
/// assert_eq!(result, 1);
///```
pub mod m_hello3{
    pub fn f_hello3() -> i32{
        println!("Hello from f_hello3");
        1
    }
}