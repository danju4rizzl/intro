pub fn run() {
    let car: Car = Car {
        name: String::from("Tesla"),
        model: String::from("TH"),
        mileage: 45000,
    };

    let car2 = Car::custom_new_car(String::from("Toyota"), String::from("RAV4"), 67000);

    car.start_engine("Key");

    car.turn_lights_on(6);

    println!(
        "Name: {}\nModel: {}\nMileage: {}",
        car2.name, car2.model, car2.mileage
    );
}

// Structures is like a JS Class
pub struct Car {
    name: String,
    model: String,
    mileage: u64,
}

impl Car {
    pub fn custom_new_car(name: String, model: String, mileage: u64) -> Self {
        Car {
            name,
            model,
            mileage,
        }
    }

    pub fn start_engine(&self, key: &str) {
        println!("{} has been egnited using key {key}", &self.name);
    }
}

impl Vehicle for Car {
    fn move_forard(&self) -> bool {
        println!("{} is moving forward", &self.name);
        true
    }

    fn reverse(&self) {
        println!("{} is reversing", self.name);
    }

    fn turn_lights_on(&self, light: u32) {
        println!("{light} {} lights are on", self.name);
    }
}

pub fn func_test() -> bool {
    return true;
}

// Traits is like TS Interface
trait Vehicle {
    fn move_forard(&self) -> bool;
    fn reverse(&self);
    fn turn_lights_on(&self, light: u32);
}
