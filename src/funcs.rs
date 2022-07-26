pub fn run() {
    let a = 34;
    let b = 78;
    let res: i32 = sum(a, b);
    println!("Sum of {a} + {b} = {}", res);

    // greeting
    let name = String::from("Paul");
    greeting(&name);

    // Closures
    let country = "Kenya";

    let func = |nation: &str| {
        // Statement
        let mut statement = String::from("My Country is: ");
        statement.push_str(nation);
        statement
    };

    println!("{}", func(&country));
}

// Declare a void function
fn sum_1(num1: i32, num2: i32) {
    // Statements
    let result = num1 + num2;
    println!("Sum of {num1} + {num2} = {}", result);
    // Return value
}

// Declare a void function
fn sum(num1: i32, num2: i32) -> i32 {
    // Statements
    let result = num1 + num2;

    // Return value
    result
}

fn greeting(name: &String) {
    println!("Hello {name}!");
}
