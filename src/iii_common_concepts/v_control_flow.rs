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

/// One of the uses of a `loop` is to retry an operation you know might fail, such as checking
/// whether a thread has completed its job. You might also need to pass the result of that
/// operation out of the loop to the rest of your code. To do this, you can add the value you want
/// returned after the `break` expression you use to stop the loop; that value will be returned out
/// of the loop so you can use it, as shown here:
pub mod repetition_with_loops {
    pub fn returning_values_from_loops() {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }

    pub fn loop_labels() {
        let mut count = 0;

        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
    }

    pub fn conditional_loops_with_while() {
        let mut number = 3;

        while number != 0 {
            println!("{number}!");

            number -= 1;
        }

        println!("LIFTOFF!!!");
    }

    pub fn looping_collections_with_for() {
        let a = [10, 20, 30, 40, 50];
        let mut index = 0;

        while index < 5 {
            println!("The value is: {}", a[index]);
            index += 1;
        }
    }

    pub fn looping_collection_concisely() {
        let a = [10, 20, 30, 40, 50];

        for element in a {
            println!("the value is: {element}");
        }
    }

    pub fn countdown_with_for() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    }
}
