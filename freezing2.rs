fn main() {
    let mut _mutable_integer = 7i32;

    {
        let mut _mutable_integer = _mutable_integer;

        _mutable_integer = 50;

        println!("{:p} has value {}", &_mutable_integer, _mutable_integer);
        // Prints 0x7ffd6d5ee7b4 has value 50
    }

    println!("{:p} has value {}", &_mutable_integer, _mutable_integer);
    // Prints 0x7ffd6d5ee7b0 has value 7
}
