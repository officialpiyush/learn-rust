// use std::fmt::Debug;

// #[derive(Debug)]
// struct Unprintable(i32);

#[derive(Debug)]
struct Printabe(i32);

fn main() {
    let p = Printabe(21);

    // println!("{}", Unprintable);
    println!("{:?}", p);
}
