pub fn run() {
    // Fixed length Strings
    let mut my_name: &str = "Deejay";
    my_name = "Paul";

    // Growable strings
    let mut sentence: String = String::from("Hello");
    sentence.push(' ');
    sentence.push_str("World");

    // Sting Methods
    let length: usize = sentence.len();

    // Capacity in Bytes
    let my_capacity: usize = sentence.capacity();

    // Searching
    let find: bool = sentence.contains("Hello");

    // Replace a Word
    let new_sentence: String = sentence.replace("World", "Paul");

    //  Check if isEmpty
    let is_empty: bool = sentence.is_empty();

    // Split a sting
    for word in sentence.split(' ') {
        println!("{}", word);
    }

    // String Cloning
    let my_first = String::from("Hello 😀");
    let my_second = my_first;

    println!("{}", my_second);
    print!(" The Empty Value is: {} ", is_empty);
}
