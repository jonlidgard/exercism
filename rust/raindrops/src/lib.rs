pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{} make?", n)
    let mut raindrop = String::new();
    if n % 3 == 0 {
        raindrop.push_str("Pling");
    }
    if n % 5 == 0 {
        raindrop.push_str("Plang");
    }
    if n % 7 == 0 {
        raindrop.push_str("Plong");
    }
    if raindrop.is_empty() {
        raindrop = n.to_string();
    }
    raindrop
}
