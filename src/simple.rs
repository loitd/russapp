pub mod m_hello{

    /// First line is for short desc
    ///
    /// Next lines for detailed documentation.
    /// still detailed. Code blocks start with triple backquotes
    ///
    /// ```
    /// let result = simple::m_hello::f_sayhello();
    /// assert_eq!(result, 1);
    ///```
    pub fn f_sayhello(){
        println!("f_sayhello");
    }
}

//-----------------------------TESTS
#[cfg(test)]
mod tests {
    
    #[test]
    fn simple_test1(){
        assert_eq!(2+2, 2*2);
    }


}