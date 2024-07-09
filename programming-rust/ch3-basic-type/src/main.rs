fn main() {
    let result = build_vector();
    println!("{:?}", result);
}

// Rust具有类型推导
pub fn build_vector() -> Vec<i16> {
    vec![10, 20]
}
