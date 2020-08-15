fn main() {
    another_function();
    func_arg(5);
    func_mult_arg(13, 25);
    expression();
    println!("return_five => {}", return_five());
    let add = return_keyword(5, 25);
    println!("Value returned by function: {}", add);
}

fn another_function() {
    println!("Another function.");
}

fn func_arg(x: i32) {
    println!("The value of x is: {}", x);
}

fn func_mult_arg(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn expression() {

    let y = {
        let x = 3;
        x + 1 //Notice thet there is no semicolon hence this is not a statement,
    };

    println!("The value of y is: {}", y);
}

fn return_five() -> i32 {
    5
}

fn return_keyword(x: i32, y: i32) -> i32 {
    let z = x + y;
    return z;
}