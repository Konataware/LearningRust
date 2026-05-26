// #TUPLES STUDY 


fn main() {
    let tup: (i32, f64, u8) = (500, 6.5, 1); // Definition of a tuple

    let (x,y,z) = tup; // Destructuring
    let five_hunnid = tup.0; // Accessing by index
    let six_dot_five = tup.1;

    let mut tup: (i32, i32) = (11, 48); // Mutable tuple
    tup.0 = tup.0 - 10;

    let out: String = tup.0.to_string();
    println!("{out}");


}
