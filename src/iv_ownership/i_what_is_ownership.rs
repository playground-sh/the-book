/// Ownership Rules
/// First, let’s take a look at the ownership rules. Keep these rules in mind as we work through
/// the examples that illustrate them:
///     * Each value in Rust has an owner.
///     * There can only be one owner at a time.
///     * When the owner goes out of scope, the value will be dropped.
pub fn variable_scope() {
    let s = "hello";
    println!("{}", s);

    {
        // s is not valid here, since it's not yet declared
        let s = "better world"; // s is valid from this point forward
        // do stuff with s
        let words: Vec<String> = s.split_whitespace().map(String::from).collect();
        println!("{:?}", words);
    } // this scope is now over, and s is no longer valid
}

/// The `String` type manages data allocated on the heap and as such is able to store an amount
/// of text that is unknown to us at compile time.
/// This kind of string can be mutated
pub fn the_string_type() {
    let mut s = String::from("hello");
    s.push_str(", World!"); // push_str() appends a literal to a String
    println!("{s}"); // this will print `hello, world!`
}

pub mod memory_and_allocation {
    /// We can probably guess what this is doing: “bind the value 5 to x; then make a copy of the
    /// value in x and bind it to y.” We now have two variables, x and y, and both equal 5. This
    /// is indeed what is happening, because integers are simple values with a known, fixed size,
    /// and these two 5 values are pushed onto the stack.
    pub fn variables_and_data_interacting_with_move() {
        let x = 5;
        println!("x: {x}");

        let y = x;
        println!("y = x = {y}");

        println!("x now: {x}");
    }

    /// Now let’s look at the `String` version
    pub fn string_with_move() {
        let s1 = String::from("hello");
        let mut refr = &s1;
        let address: *const String = refr;
        println!("s1: {s1}, address of s1: {address:?}");

        let s2 = s1; // value moved here
        refr = &s2;
        let address: *const String = refr;
        println!("s2: {s2}, address of s2: {address:?}");

        // CAN'T do this now. `s2` now points to the memory location pointed to by `s1`
        // (where the String `hello` lives). `s1` is now freed and points to nothing therefore it's
        // an invalid reference.
        // println!("s1: {s1}");   // value borrowed here after move
    }

    pub fn scope_and_assignment() {
        let mut s = String::from("Hello");
        let mut refr = &s;
        let address: *const String = refr;
        println!("s1: {s}, address of s: {address:?}");

        s = String::from("ahoy");
        refr = &s;
        let address: *const String = refr;
        println!("s1: {s}, address of s: {address:?}");

        println!("{s}, world!");
    }

    pub fn variables_and_data_interacting_with_clone() {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");

        let mut refr = &s1;
        let address: *const String = refr;
        println!("s1: {s1}, address of s1: {address:?}");

        refr = &s2;
        let address: *const String = refr;
        println!("s2: {s2}, address of s2: {address:?}");
    }
}

pub mod ownership_and_copy {
    pub fn ownership_copy() {
        let s = String::from("hello"); // s comes into scope
        // s's value moves into the function...
        takes_ownership(s);
        // ... and so is no longer valid here
        // println!("{s}");

        let x = 5; // x comes into scope
        // Because i32 implements the Copy trait,
        // x does NOT move into the function,
        makes_copy(x);
        // so it's okay to use x afterward.
        println!("{x}");
    }

    fn takes_ownership(some_string: String) {
        println!("{some_string}");
    }

    fn makes_copy(some_integer: i32) {
        println!("{some_integer}");
    }
}

pub mod return_values_and_scope {
    pub fn scope() {
        // gives_ownership moves its return value into s1
        let s1 = gives_ownership();
        println!("{s1}");

        let s2 = String::from("hello"); // s2 comes into scope

        // s2 is moved into
        // takes_and_gives_back, which also
        // moves its return value into s3
        let s3 = takes_and_gives_back(s2);
        println!("{s3}");
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

    // `gives_ownership` will move its
    // return value into the function
    // that calls it
    fn gives_ownership() -> String {
        let s = String::from("yours"); // `s` comes into scope
        // `s` is returned and
        // moves out to the calling
        // function
        s
    }

    // This function takes a String and returns a String.
    fn takes_and_gives_back(val: String) -> String {
        // `val` comes into
        // scope
        val // `val` is returned and moves out to the calling function
    }

    /// While this works, taking ownership and then returning ownership with every function is a
    /// bit tedious. What if we want to let a function use a value but not take ownership? It’s
    /// quite annoying that anything we pass in also needs to be passed back if we want to use it
    /// again, in addition to any data resulting from the body of the function that we might want
    /// to return as well.
    pub fn return_multiple_values() {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);
        println!("The length of '{s2}' is {len}.");
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }
}
