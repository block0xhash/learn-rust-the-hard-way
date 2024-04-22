
fn main() {
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.into_iter() {
        match name {
            "Ferris" => names.push("New Name"),
            _ => println!("Hello {}", name),
        }
    }
    
    // println!("names: {:?}", names);
    // FIXME ^ Comment out this line
}
