fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    // Using get with explicit bounds checking
    match vec.get(1) {
        Some(value) => println!("The value is: {value}"),
        None => println!("Index out of bounds"),
    }
    //Alternative safer approach
    if let Some(value) = vec.get(1){
        println!("The value is: {value}");
    } else {
        println!("Index out of bounds");
    }
}