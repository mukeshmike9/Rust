fn main() {
    return_out_of_sync();
    string_slices();
    let string = String::from("Hello World");
    let first = first_word_as_slice(&string);
    println!("First Word: {}", first);
    array_slice();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

/**
 * Error Case
 */
fn return_out_of_sync() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
    println!("first_word return: {}", word);
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    //The word is out of sync
}

fn string_slices() {
    let s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..11];
    //This is same as previous line
    //let world = &s[6..];

    println!("1st slice: {}, 2nd slice: {}", hello, world);
}

/**
 * Pass STring as slice
 */
fn first_word_as_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/**
 * Pass array as slice
 */
fn array_slice() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    let mut j = 0;
    for i in slice {
        println!("slice[{}]: {}", j, i);
        j += 1;
    }
}
