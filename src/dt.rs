/**
 * Primitive Types:--
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, (number of bits they take in memory)
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 */

// Rust is a statically typed language, which means that it must know the types of all variables at compiletime, however, the compiler can usually infer what type we want to use based on the value and how we use it.

pub fn run() {
    let my_x: u8 = 7;
    let z: i32 = -3;
    let y: u8 = 5;

    // float
    let is_float: f64 = 48.3;

    // Boolean
    let is_cold: bool = false;
    let is_valid: bool = my_x > y;
    // Character
    let c: char = 'c';

    // Tuple
    let cities: (&str, &str, u64) = ("Cape Town", "Nairobi", 60);

    // println!("{}", c);
    println!(
        "{city_2},{city_1} are cities in with a population of {population}",
        city_1 = cities.0,
        city_2 = cities.1,
        population = cities.2
    );
}
