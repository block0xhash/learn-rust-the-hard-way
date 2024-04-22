pub mod numbers {

    pub fn say_hello() {
        println!("hello, world! from demo");
    }

    pub fn print() {
        let numbers = vec![1, 2, 3, 4, 5];
        for n in numbers {
            println!("{}", n);
        }
    }
}
