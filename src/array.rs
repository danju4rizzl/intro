pub fn run() {
    // Fixed length arrays of some data types
    let mut class: [&str; 3] = ["Paul", "Deejay", "Tony"];
    class[1] = "stray";
    println!("{:?}", class[1]);

    let mut ages: Vec<u64> = vec![];
    ages.push(39);
    ages.push(78);
    ages.push(37);
    ages.push(98);
    // ages.pop();
    // Remove specific idx
    // ages.remove(1);

    println!("{:?}", ages[3])
}
