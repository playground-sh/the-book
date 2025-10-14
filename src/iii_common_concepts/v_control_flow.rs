pub mod if_expressions {
    pub fn basic() {
        let number = 3;

        if number < 5 {
            println!("Condition was true!")
        } else {
            println!("Condition was false!")
        }
    }

    pub fn else_if() {
        let number = 6;

        if number % 4 == 0 {
            println!("number is divisible by 4");
        } else if number % 3 == 0 {
            println!("number is divisible by 3");
        } else if number % 2 == 0 {
            println!("number is divisible by 2");
        } else {
            println!("number is not divisible by 4, 3, or 2");
        }
    }

    pub fn using_if_in_a_let_statement() {
        let condition = true;
        let number = if condition { 5 } else { 6 };

        println!("The value of number is: {number}");
    }
}
