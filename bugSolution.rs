fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6

    //The following is now valid as long as no other immutable references exist
    let z = &mut x; // z is a mutable reference to x
    *z += 1; // Modifying x through z is allowed
    println!("x = {}", x); // Output: x = 7
} 