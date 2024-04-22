fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // sets counter to 2 and exits the loop
        }
    };

    assert_eq!(result, 200);
}
