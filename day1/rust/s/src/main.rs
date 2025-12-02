fn manufacture_gifts(gifts_to_produce: Vec<String>) -> Vec<String> {
    gifts_to_produce
        .into_iter()                        // 1. Consume el Vec y produce un iterador de String
        .filter(|gift| !gift.contains('#'))// 2. Filtra: queda solo si NO contiene '#'
        .collect()                          // 3. Recolecta los resultados en un nuevo Vec<String>
}


fn main() {
        let gifts1 = vec![
        "car".to_string(),
        "doll#arm".to_string(),
        "ball".to_string(),
        "#train".to_string(),
    ];
}

let good1 = manufacture_gifts(gifts1);
println!("{:?}", good1); 
