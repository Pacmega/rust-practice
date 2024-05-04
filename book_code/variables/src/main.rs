fn main() {
    let x;
    println!("x is immutable.");
    x = 5;
    println!("The value of x is: {x}");
    let mut changeable_var = 2; // Implicit type
    let floaty_boi: f32 = 2.2; // Explicit type
    println!("The value of changeable_var is: {changeable_var}");
    changeable_var = 4;
    println!("And now it is: {changeable_var}");
}
