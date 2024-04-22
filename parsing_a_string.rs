// to convert a string to a number
// the idiomatic approach is to use the parse function
// either arrange for type inference or
// specify the type to parse turbofish syntax

// This will convert the string into the type specified as long as the FromStr trait is implemented for that type
// for user defined type simply implement the FromStr trait for that type

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
