pub fn run() {
    let mut name: &str = "Deejay";
    let country: &str = "South Africa";
    let profession: &str = "Web3 Developer ";

    print!("{} is a {} from {}", name, profession, country);
    name = "Paul";

    print!("{} is a {} from {}", name, profession, country);

    // Constants definitions
    const PIE: f32 = 3.142;
    print!("{}", PIE);
}
