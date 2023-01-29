use crate::valuator::Valuator;
use std::env;

mod valuator;

#[cfg(test)]
mod valuator_tests;

fn main() {
    let args: Vec<String> = env::args().collect();

    let thing = Valuator::new();

    // todo add validation
    let input = &args[1];
    let output = thing.evaluate(input);

    println!("The value of {input} is {output}");
}
