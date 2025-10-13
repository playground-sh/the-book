pub fn int_types() {
    // 8-bit signed and un-signed
    let i8_max: i8 = i8::MAX; // 127
    let i8_min: i8 = i8::MIN; // -128
    let u8_max: u8 = u8::MAX; // 255
    println!("i8 max: {}\ni8 min: {}\nu8 max: {}", i8_max, i8_min, u8_max);

    // 16-bit signed and un-signed
    let i16_max: i16 = i16::MAX; // 32_767
    let i16_min: i16 = i16::MIN; // -32_768
    let u16_max: u16 = u16::MAX; // 65_535
    println!(
        "i16 max: {}\ni16 min: {}\nu16 max: {}",
        i16_max, i16_min, u16_max
    );

    // 32-bit signed and un-signed
    let i32_max: i32 = i32::MAX; // 2_147_483_647
    let i32_min: i32 = i32::MIN; // -2_147_483_648
    let u32_max: u32 = u32::MAX; // 4_294_967_295
    println!(
        "i32 max: {}\ni32 min: {}\nu32 max: {}",
        i32_max, i32_min, u32_max
    );

    // 64-bit signed and un-signed
    let i64_max: i64 = i64::MAX; // 9_223_372_036_854_775_807
    let i64_min: i64 = i64::MIN; // -9_223_372_036_854_775_808
    let u64_max: u64 = u64::MAX; // 18_446_744_073_709_551_615
    println!(
        "i64 max: {}\ni64 min: {}\nu64 max: {}",
        i64_max, i64_min, u64_max
    );

    // 128-bit signed and un-signed
    let i128_max: i128 = i128::MAX; // 170_141_183_460_469_231_731_687_303_715_884_105_727
    let i128_min: i128 = i128::MIN; // -170_141_183_460_469_231_731_687_303_715_884_105_728
    let u128_max: u128 = u128::MAX; // 340_282_366_920_938_463_463_374_607_431_768_211_455
    println!(
        "i128 max: {}\ni128 min: {}\nu128 max: {}",
        i128_max, i128_min, u128_max
    );

    // isize & usize
    println!(
        "isize max: {}\nisize min: {}\nusize max: {}",
        isize::MAX,
        isize::MIN,
        usize::MAX
    );
}

pub fn int_literals() {
    let decimal_literal = 98_222;
    let hex_literal = 0xfff;
    let octal_literal = 0o77;
    let binary_literal = 0b1111_0000;
    let byte_literal = b'A';

    println!("decimal_literal (98_222): {decimal_literal}");
    println!("hex_literal (0xfff): {hex_literal}");
    println!("octal_literal (0o77): {octal_literal}");
    println!("binary_literal (0b1111_0000): {binary_literal}");
    println!("byte_literal (b'A'): {byte_literal}");
}

pub fn floating_point_types() {
    let f32_min: f32 = f32::MIN;
    let f32_max: f32 = f32::MAX;
    println!(
        "f32_min: {f32_min}\n\
         f32_max: {f32_max}"
    );

    let f64_min: f64 = f64::MIN;
    let f64_max: f64 = f64::MAX;
    println!(
        "f64_min: {f64_min}\n\
         f64_max: {f64_max}"
    );
}

pub fn boolean_types() {
    let truthy = true;
    let falsy: bool = false;

    println!("Truthy is: {truthy} and Falsy is: {falsy}");
}

pub fn character_type() {
    let c = 'z';
    let z: char = 'Z';
    let emoji = '😺';

    println!("c: {c}, z: {z}, emoji: {emoji}");
}

pub fn tuple_type() {
    let tup = (500, 3.14, '𝜋', true, "oh crab!");

    // tuple unpacking
    let (int, float, character, boolean, string) = tup;
    println!(
        "int: {int}\n\
         float: {float}\n\
         character: {character}\n\
         boolean: {boolean}\n\
         string: {string}\n"
    );

    // tuple indexing
    let num = tup.0;
    let char = tup.2;
    let string_literal = tup.4;
    println!(
        "num: {num}\n\
         char: {char}\n\
         string_literal: {string_literal}"
    );
}
