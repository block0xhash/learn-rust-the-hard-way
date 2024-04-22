// casting converts between primitive types
// traits are used to convert between custom types
// generic conversions will use the From and Into traits
// The From and Into traits are linked, and is part of the implementation
// If you are able to convert from type A to B then conversion from B to A should be possible

// From trait allows for a type to define how to create itself from another type
// , hence providing a very simple mechanism for converting between several types
// There are numerous implementations of this trait within the std lib
//
// let my_str = "hello";
// let my_string = String::from(my_str)

// Into trait is simply a reciprocal of the From trait
// If you have implemented the From trait for your  type
// Into will call it when necessary

use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

/*
impl Into<Number> for i32 {
    fn into(self) -> Number {
        Number { value: self }
    }
}
*/

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num.value);

    let int = 5;
    // Try removing the type annotation
    let num = int.into();
    println!("My number is {:?}", num);
}
