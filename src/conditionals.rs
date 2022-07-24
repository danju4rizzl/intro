pub fn run() {
    //  If Statement
    if 7 > 10 {
        println!("is grater than 🔟");
    }

    // If else Statement
    if 7 > 10 {
        println!("is grater than 🔟");
    } else {
        println!("is less than 🔟");
    }

    // If Else if statement
    if 7 > 10 {
        println!("is grater than 🔟");
    } else if 6 < 8 {
        println!("is less than 8️⃣");
    } else {
        println!("is less than 🔟");
    }

    //  Match equivalent to js switch
    let age: i32 = 8;
    match age {
        50 => {
            // Block statement
            println!("Above  50 👍");
        }
        10 => println!("Is 🔟"),
        8 => println!("Is 8️⃣"),

        _ => {
            //The _ is the default when if nothing passes
            println!("Nothing happened");
        }
    }
}
