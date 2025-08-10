fn main() {
    let string_to_analyze = String::from("Hello, Dolly");
    let index_of_first_word_found = get_words(&string_to_analyze);
    
    if let Some(first_word) = string_to_analyze.get(..index_of_first_word_found) {
        println!("The first word is {}", first_word);
    } else {
        println!("There was an error...")
    }
    
}

fn get_words(string: &String) -> usize {

    let string_as_bytes = string.as_bytes();

    for (counter, &item) in  string_as_bytes.iter().enumerate() {
        if item == b' ' {
            return counter;
        }
    }
    
    string.len()
}


/* 
fn main() {
    let mut s = String::from("hello world");
    let s2 = "hello world";

    /* 
    // string slices using first_word function
    let slice_hello = &s[..first_word(&s)];
    let slice_world = &s[first_word(&s)+1..];
    println!("First word is {} and second word is {}", slice_hello, slice_world);
    */


    // string slices using first_word_slices function
    
    let word = first_word_slices(s2);
    s.clear();
}
 */
/* 
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
 */
/* 
fn first_word_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
 */