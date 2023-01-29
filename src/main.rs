use crate::valuator::Valuator;

mod valuator;

#[cfg(test)]
mod valuator_tests;


fn main() {
    println!("Hello, world!");

    let thing = Valuator::new();
}
