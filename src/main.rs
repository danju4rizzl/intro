// mod variables;
// mod dt;
// mod array;
// mod strings;
// mod conditionals;
// mod loops;
// mod funcs;
// mod enums;
mod traits;

fn main() {
    // variables::run();
    // dt::run();
    // array::run();
    // strings::run();
    // loops::run();
    // funcs::run();
    // enums::run();
    traits::run();
}

#[cfg(test)]

mod tests {
    // using the " Use super::* " to import all the main models for testing an external file.
    use super::*;

    #[test]
    pub fn test_sum() {
        // assert!()
        // assert_eq!()
        // assert_ne!()

        let a = 7;
        let b = 3;

        assert!(a + b == 10, "Test Failed");
    }

    #[test]
    pub fn test_string() {
        let name = "Deejay";

        assert_eq!(name, "Deejay");
    }

    #[test]
    pub fn test_nums() {
        assert_ne!(9, 19);
    }

    #[test]
    pub fn test_func() {
        let res = traits::func_test();
        assert_eq!(res, false);
    }
}
