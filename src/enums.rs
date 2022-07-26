pub fn run() {
    let country: Countries = Countries::SOUTH_AFRICA;
    let states: Countries = Countries::Places {
        states: String::from("Cape Town"),
    };

    println!("{:?}", states);

    let option: MyOption = MyOption::None;
    match option {
        MyOption::MySome(val) => println!("{val}"),
        MyOption::None => println!("Nothing here 😱"),
    }
}

#[derive(Debug)]
enum Countries {
    NIGERIA,
    KENYA,
    SOUTH_AFRICA,
    Places { states: String },
}

enum MyOption {
    MySome(i32),
    None,
}
