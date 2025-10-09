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

pub fn constants() {
    const EARTHS_GRAVITATIONAL_PULL: f32 = 9.81;
    const SPEED_OF_LIGHT: i32 = 299_792_458;

    println!(
        "The earth's gravitational pull is {} m/s^2",
        EARTHS_GRAVITATIONAL_PULL
    );
    println!("The speed of light is {} m/s", SPEED_OF_LIGHT);
}

pub fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
