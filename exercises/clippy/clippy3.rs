// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

// ~I AM NOT DONE

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    
    // This is a logical error; unwrapping a None will panic. 
    // Instead, we can just check it and do nothing.
    if my_option.is_none() {
        // Do nothing, as unwrapping None would panic
    }

    // Fixing the array declaration by adding a comma between elements.
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Using `Vec::with_capacity` instead of `resize` to create an empty vector.
    let my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5].into_iter().take(0).collect();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two using std::mem::swap
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
