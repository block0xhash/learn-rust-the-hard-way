// implementing fmt::Display
// Import (via `use`) the `fmt` module to make it available
use std::fmt;

// Derive (automatically create) the `fmt::Debug` implementation for `Structure`.
// `Structure` is a structure which contains a single `i32`.

#[derive(Debug)] 
struct Structure(i32);

// to use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    
    // trait requires `fmt` with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: f.
        write!(f, "{}",self.0)

    }
}

// Put a `Structure` inside of the structure `Deep`.  Make it printable also.
#[derive(Debug)]
struct Deep(Structure);
impl fmt::Display for Deep {
    
    // trait requires `fmt` with this exact signature
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output stream: f.
        write!(f, "{}",self.0)
    }
}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// A structure holding two numbers. `Debug` will be derived so the results can be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`.
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}



fn main() {
    
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);

    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` is printable!
    println!("Display: {} will print!", Structure(3));
    println!("Debug: {:?} will print!", Structure(3));

    // The problem with `derive` is there is no control over how
    // the results look. What if I want this to just show a `7`?
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Pretty print
    println!("{:#?}", peter);

        let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);





}
