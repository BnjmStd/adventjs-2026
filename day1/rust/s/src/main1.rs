fn manufacture_gifts(gifts_to_produce: &[&str]) -> Vec<String> {
    gifts_to_produce
        .iter()                         // 1. Iterador sobre &str
        .filter(|&&gift| !gift.contains('#')) // 2. Filtramos por ausencia de '#'
        .map(|&gift| gift.to_string())  // 3. Convertimos cada &str a String (clonando el texto)
        .collect()                      // 4. Recolectamos en Vec<String>
}

fn main() {
    let gifts1 = ["car", "doll#arm", "ball", "#train"];
    let good1 = manufacture_gifts(&gifts1);
    println!("{:?}", good1); // ["car", "ball"]
    // gifts1 sigue existiendo porque no fue consumido
}
