fn main() {
    borrowing_ownership();
    mutable_references();
    combining_mutable_immutable_reference();
    let string = no_dangle();
    println!("no_dangle string: {}", string);
}

fn borrowing_ownership() {
    let s1 = String::from("hello");
    //Passing s1 as reference means that we are not giving up the ownership
    //The ownership still resides with the calling function
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {

    //Any attempt to modify borrowed object here will cause compiler error
    //As this is still immutable
    //e.g. try uncommenting the next line
    //s.push_str("My World"); 
    s.len()
}

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    //This change here is successful as the reference is mutable
    //Problem is you can only have one mutable reference at a time 
    //This is to prevent giving write access from multiple points 
    some_string.push_str(", world");
}


fn combining_mutable_immutable_reference() {
        let mut s = String::from("hello");
    
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        //Uncommenting next line will cause a compiler error
        //let r4 = &mut s;
        println!("{} and {}", r1, r2);
        // r1 and r2 are no longer used after this point
    
        let r3 = &mut s; // no problem
        println!("{}", r3);
}


fn no_dangle() -> String {
    let s = String::from("hello");

    //This will trnsfer the ownership to the caller
    s
    //Uncommenting next line will cause compiler error as the current function will end and give up ownership of s
    //This will leave it dangling
    //&s
}
