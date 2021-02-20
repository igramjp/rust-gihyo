use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);
    println!("Hello, world!");
    println!("Hello, {}!", "Manabu");
    println!(
        "半径 {:.1}，円周率 {:.3}，面積 {:.3}",
        3.2,
        std::f64::consts::PI,
        3.2f64.powi(2) * std::f64::consts::PI
    );
}
