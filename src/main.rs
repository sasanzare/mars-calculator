use std::io;

fn main() {
    println!("Please Enter Your Weight (KG)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("weight on Mars: {} KG ", calculate_weight_on_mars(input.trim().parse().unwrap()));
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}