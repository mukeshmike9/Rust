fn main() {
    tuple_type();
    array_type();
}

fn array_type() {
    println!("initializin an array");
    //Array in Rust have fixed length
    let x = [1, 2, 3, 4, 5];
    let mut i = 0;
    loop {
        //This is how we access an element from array
        println!("x[{}] = {}", i, x[i]);
        i += 1;
        if i > 4
        {
            break;
        }
    }
    println!("Defalt value of array");
    //this will create array of size five with 3 as each element
    let a = [3; 5];
    i = 0;
    for x in &a {
        println!("a[{}] = {}", i, x);
        i += 1;
    }
}

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.1, 127);

    //Accessing tuple
    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);

    //Another way to access Tuple
    println!("tup 0 = {}, tup 1 = {}, tup 2 = {}, ", tup.0, tup.1, tup.2);
}
