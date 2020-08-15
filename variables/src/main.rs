
fn main() {
    let x = 5;
    println!("x = {}", x);
    /*
     * x = 6; will cause an error
     * Using let will shadow the previous assignment of x and new value will be used
     */
    let x = 6; //let keyword is important as x is immutable by default

    const CONST_VALUE : u32 = 100; //You cannot change its type or value 

    println!("x = {}, CONST_VALUE = {}", x, CONST_VALUE);
}

