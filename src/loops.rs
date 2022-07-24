pub fn run() {
    // Infinite LOOPS
    let mut my_count: i32 = 0;

    loop {
        println!("Infinity and Beyond {my_count} ➰ ");
        if my_count > 10 {
            break;
        }
        my_count += 1;
    }

    // Example using While Loop in a function
    run_while_loop(10);

    //  For Loop with in a Range iterator
    for i in 0..10 {
        println!("{}", i * 2);
    }

    //  Example using a For Loop with in a Array iterator in a function
    run_for_loop();
}

fn run_for_loop() {
    for j in [20, 2, 0, 312, 323, 5, 35, 42] {
        if j > 100 {
            println!("🟢 {} is more than 100 ", j)
        } else if j < 100 && j > 10 {
            println!("🟡 {j} is not much ")
        } else {
            println!("🔴 Nothing to see here ")
        }
    }
}

fn run_while_loop(counter: i32) {
    let mut my_count: i32 = 0;

    while my_count < counter {
        println!("While age is {my_count} 🎈");
        if my_count == 3 {
            println!("Third: 3️⃣");
        }
        my_count += 1;
    }
}
