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