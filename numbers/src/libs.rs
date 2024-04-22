pub mod numbers {

    pub fn say_hello() {
        println!("hello, world! from libs");
    }

    pub fn print(limit: u8) {
        let numbers = generate_sequence(limit);
        output_sequence(&numbers);
        output_sequence(&numbers);
        let array_numbers = [1, 2, 3, 4, 5];
        output_sequence(&array_numbers);
    }

    fn output_sequence(numbers: &[u8]) -> () {
        for n in numbers.iter() {
            println!("{}", n);
        }
    }

    fn generate_sequence(limit: u8) -> Vec<u8> {
        (1..=limit).collect::<Vec<u8>>()
    }

    #[test]
    fn generate_sequence_should_work() {
        let result = generate_sequence(3);
        assert_eq!(result, &[1,2,3]);
    }
}
