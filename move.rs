
fn main() {
    let a = 5; // Declare a variable 'a' and assign the value 5
    let b = a; // Create a new variable 'b' and assign the value of 'a'

    // Uncommenting the next line will result in a compilation error:
    println!("a: {}", a);

    println!("b: {}", b); // Prints the value of 'b'
                          //

    let v = vec![1,2,3];
    //let w=v;
    println!("{:?}", &v);
}

