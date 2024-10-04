// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator. Please note
// that the `as` operator is not only used when type casting. It also helps with
// renaming imports.
//
// The goal is to make sure that the division does not fail to compile and
// returns the proper type.
//
// Execute `rustlings hint using_as` or use the `hint` watch subcommand for a
// hint.

// ~I AM NOT DONE

fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0; // Return 0.0 or another appropriate value for empty input
    }
    
    let total = values.iter().sum::<f64>();
    total / values.len() as f64 // Cast len() to f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }

    #[test]
    fn returns_zero_for_empty_slice() {
        assert_eq!(average(&[]), 0.0); // Test for empty input
    }
}
