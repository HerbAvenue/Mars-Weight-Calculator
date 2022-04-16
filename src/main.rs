use std::io;

fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let user_weight: f32 = input.trim().parse().unwrap();


    let calculated_weight = calculate_mars_weight(user_weight);
    
    println!("Mars Weight: {}", calculated_weight);   
}

fn calculate_mars_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}