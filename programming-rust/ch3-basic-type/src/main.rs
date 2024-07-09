fn main() {
    let result = build_vector();
    println!("{:?}", result);
}

// Rust具有类型推导
pub fn build_vector() -> Vec<i16> {
    let mut v = vec![]; // 所以这里就不需要给v声明类型
    v.push(10); // 这里声明了push 一个 i16的类型
    v.push(20);
    v
}
