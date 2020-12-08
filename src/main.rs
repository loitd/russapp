// this is the example from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use std::io;
mod simple; // declare module simple (file simple.rs) to be used - must
// declare a public function in the crate to be used
pub use crate::simple::m_hello::f_sayhello; 

// declare to use mod sim. Mod sim will be declare in simp.rs file + sub-mod in separated files inside simp folder.
mod simp;
pub use crate::simp::m_hello2::f_sayhello2;

// use a mod inside lib.rs file. This is app scope 
// -> rlib name = lib + appname + .rlib -> call by app name just like extern crate
pub use russapp::m_hello3::f_hello3;

// use extern crate from crate.io (using cargo). Please remind that simplelog needs log's macros
#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;


fn main() {
    f_sayhello(); // call the public imported function
    f_sayhello2(); // call the public imported n
    f_hello3();

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("russapp.log").unwrap()),
        ]
    ).unwrap();

    trace!("This is trace");
    warn!("This is warn");
    error!("This is error");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
