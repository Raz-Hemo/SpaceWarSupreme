use std::fmt::Display;

pub fn print_console<T: Display>(x: &T) -> () {
    println!("{}", x)
}
