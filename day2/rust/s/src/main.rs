
#[derive(Debug)]
struct Production {
    toy: String,
    quantity: u32,
}

fn solution(production: Vec<Production>) -> u32 {
    // Day 2 solution

    production.iter().map(|p| p.quantity).sum()
}

fn main() {
    
    let production1 = vec![
        Production { toy: "car".to_string(), quantity: 3 },
        Production { toy: "doll".to_string(), quantity: 1 },
        Production { toy: "ball".to_string(), quantity: 2 },
    ];

    println!("{}", solution(production1)); 
}
