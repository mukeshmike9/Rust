fn main() {
    integer_numbers();
    floating_point_numbers();
    boolean();
    char_type();
}

fn floating_point_numbers() {
    let x = 2.9; // f64

    let y: f32 = 3.4; // f32

    println!("x = {}, y = {}", x, y);
}

fn integer_numbers() {
    let a = 4; // Simple Integer
    let b: u64 = 98_222; // _ is only visual representation and is ignored by compiler, u64 signifies 64 bit memory will be allocated
    let c: u32 = 0x00AB; //Hexadecimal representation 171
    let d: u32 = 0o76; //Octal representation 62
    let e: u32 = 0b0000_1111_0101; //Binary representation of 245

    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
    println!("a = {:b}, b = 0x{:x}, c = 0x{:X}, d = {:o}, e = 0b{:0>5$b}", a, b, c, d, e, 12);
}

fn boolean() {
    let t = true;

    let f: bool = false; // with explicit type annotation

    println!("t = {}, f = {}", t, f);
}

fn char_type() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c = {}, z = {}, heart_eyed_cat = {}", c, z, heart_eyed_cat);
}
