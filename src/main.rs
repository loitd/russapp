// this is the example from https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
use std::io;
mod simple; // declare module simple (file simple.rs) to be used - must
// declare a public function in the crate to be used
pub use crate::simple::m_hello::f_sayhello; 

// declare to use mod sim. Mod sim will be declare in simp.rs file + sub-mod in separated files inside simp folder.
mod simp;
pub use crate::simp::m_hello2;

// use a mod inside lib.rs file. This is app scope 
// -> rlib name = lib + appname + .rlib -> call by app name just like extern crate
pub use russapp::m_hello3::f_hello3;

// use extern crate from crate.io (using cargo). Please remind that simplelog needs log's macros
#[macro_use] extern crate log;
extern crate simplelog;
use simplelog::*;
use std::fs::File;

// for multithreading
use std::thread;

// use threadpool
extern crate threadpool;
use threadpool::ThreadPool;

// use mysql -> we will use diesel for ORM in RUST. Diesel supports MySQL, PG, SQLite
// https://docs.diesel.rs/master/diesel/connection/trait.Connection.html
mod db;
pub use crate::db::dbmysql::test;

fn main() {
    f_sayhello(); // call the public imported function
    m_hello2::f_sayhello2(); // call the public imported n
    f_hello3();
    println!("MUL is: {}", m_hello2::mul(10,"2"));

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("russapp.log").unwrap()),
        ]
    ).unwrap();

    trace!("This is trace");
    warn!("This is warn");
    error!("This is error");

    // spawn a new thread
    let t1 = thread::spawn(move || {
        println!("Hello, I am from another thread");
    });

    // sleep for a while
    // thread::sleep(2000);

    // use oin() to connect to the t1 thread + unwrap() to get the result
    let result = t1.join().unwrap();
    println!("{:?}",result);

    // Using threadpool
    let pool = ThreadPool::new(10); //declare 10 threads
    for i in 0..10{
        pool.execute(move || {
            println!("Creating thread {} using pool", i);
        });
    }

    // Rust giữ cơ chế bảo mật luồng, nếu x được khai báo tại hàm main và các thread thay đổi giá trị của x thì cuối cùng, x tại main vẫn k đổi và các thread chỉ đang làm việc với các bản sao của x. Để có thể chia sẻ dữ liệu trên luồng thì Rust cung cấp 1 kiểu dữ liệu với tên gọi std::sync::atomic như Arc, Mutex, RwLock, ...

    // connect to mysql
    test();

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
