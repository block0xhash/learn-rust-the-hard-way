fn main() {
    // 1. //  Line comments ( recommended comment style )
    // 2. /*  Block start comment - Blockend comment */ ( useful for commenting chunks of code )

    // 3. /// Generate HTML library docs for the following item
    // 4. //! Generate HTML library docs for the enclosing item

    //---------------------------------------------------------

    // 1. In general, the `{}` will be automatically replaced with
    //    any arguments. These will be stringfied
    //    ONLY types that implement fmt::Display can be formatted with {}
    //    User definded types do not implement fmt::Display by default.

    println!("{} days", 31);

    // 2. Positional arguments can be used. Specifying an integer inside {}
    //    determines which additional arguments will be replaced
    //    arguments start at 0 immediately after the format string
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 3. named arguments
    println!(
        "{subject} {verb} {object} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 4. specify format character after a `:`
    println!("Base 10:                 {}", 69420);
    println!("Base 2 (binary):         {:b}", 69420);
    println!("Base 8 (octal):          {:o}", 69420);
    println!("Base 16 (hexadecimal):   {:x}", 69420);

    // 5. right-justify with a specified width
    //    (outputs 4 spaces and a "1" for a total width of 5)
    println!("{number:>5}", number = 1); // "    1"

    // 6. pad numbers with extra zeroes
    println!("{number:0>5}", number = 1); // "00001"

    // 7. left-adjust by flipping the sign
    println!("{number:0<5}", number = 1); // "1000"

    // 8. to use named aguments in format specifier append a `$`
    println!("{number:0>width$}", number = 1, width = 15);

    // 9. aguments can be captured from surrounding variables
    let number: f64 = 1.0;
    let width: usize = 10000;
    println!("{number:9>width$}");

    // Activities
    //
    println!(" My name is {0} {1}, {1} {0}  ", "James", "Bond"); // added missing agument "James"

    #[allow(dead_code)] // disable dead code that warns against unused module
    struct Structure(i32);

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    let pi = 3.141592;
    println!("Pi is roughly {pi:.3}"); // Pi is roughly 3.142
}
