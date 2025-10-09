pub fn mutability_broken() {
    let x = 5;
    println!("The value of x is: {x}");

    // The following line will not compile
    // x = 6;
    println!("The value of x is: {x}");
}

pub fn mutability_fixed() {
    let mut x = 100;
    println!("The value of x is: {x}");

    // This works fine
    x = 200;
    println!("The value of x is: {x}");
}
