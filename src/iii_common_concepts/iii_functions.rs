pub fn with_parameter(x: i32) {
    // here `x` is the parameter
    println!("The value of the parameter x is {x}");
}

pub fn multiple_parameters(value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}

pub fn statements_and_expression() {
    // let y = 6; is a statement.
    let y = 6;
    // the 6 in the statement let y = 6; is an expression that evaluates to the value 6

    // A new scope block created with curly brackets is an expression
    let y = {
        let x = 3;
        x + 1
    };
    /// This expression:
    /// {
    ///     let x = 3;
    ///     x + 1
    /// }
    /// is a block that, in this case, evaluates to 4.
    /// That value gets bound to `y` as part of the let statement.
    /// Note that the `x + 1` line doesn’t have a semicolon at the end, which is unlike most
    /// of the lines you’ve seen so far. Expressions do not include ending semicolons. If
    /// you add a semicolon to the end of an expression, you turn it into a statement, and
    /// it will then not return a value. Keep this in mind as you explore function return
    /// values and expressions next.

    println!("The value of y is: {y}");
}

// Functions with Return Values
pub fn five() -> i32 {
    5
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}
