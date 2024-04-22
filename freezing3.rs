fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _mutable_integer = &mut _mutable_integer;

        *_mutable_integer = 50;

        println!("{:p} has value {}", _mutable_integer, _mutable_integer);
        // Prints 0x7ffe5ccc6b14 has value 50
    }

    println!("{:p} has value {}", &_mutable_integer, _mutable_integer);
    // Prints 0x7ffe5ccc6b14 has value 50
}
