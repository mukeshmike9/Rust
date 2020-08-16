fn main() {
    returning_value_from_loops();
    while_loop();
    for_loop();
    for_loop_with_range();
    for_loop_with_range_in_reverse();
}

fn returning_value_from_loops() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            //Without this statement this will be an infinite loop
            //Any expression after break will be returned from loop
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn while_loop() {
    let mut number = 10;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn for_loop_with_range() {
    //Number count 1 to 10
    for number in 1..11 {
        println!("{}!", number);
    }
}

fn for_loop_with_range_in_reverse() {
        for number in (1..11).rev() {
            println!("{}!", number);
        }
        println!("LIFTOFF!!!");
    }
    