use std::fs::File;
use std::io::Write;

pub fn saye_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"Hello world")?;
    out.flush()
}

pub fn dot_product<const N: usize>(a: [f64; N], b: [f64; N]) -> f64 {
    let mut result = 0.0;
    for i in 0..N {
        result += a[i] * b[i];
    }
    result
}

trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        let c = *self as u32;

        // https://en.wikipedia.org/wiki/Emoji#Unicode_blocks
        (c >= 0x1F600 && c <= 0x1F64F)
            || (c >= 0x1F300 && c <= 0x1F5FF)
            || (c >= 0x1F680 && c <= 0x1F6FF)
            || (c >= 0x1F700 && c <= 0x1F77F)
            || (c >= 0x1F780 && c <= 0x1F7FF)
            || (c >= 0x1F800 && c <= 0x1F8FF)
            || (c >= 0x1F900 && c <= 0x1F9FF)
            || (c >= 0x1FA00 && c <= 0x1FA6F)
            || (c >= 0x2600 && c <= 0x26FF)
            || (c >= 0x2700 && c <= 0x27BF)
            || (c >= 0x2300 && c <= 0x23FF)
            || (c >= 0x2B50 && c <= 0x2B5F)
            || (c >= 0x2B06 && c <= 0x2B07)
            || (c >= 0x2B50 && c <= 0x2B5F)
            || (c >= 0x2934 && c <= 0x2935)
            || (c >= 0x3297 && c <= 0x3299)
    }
}

fn main() -> std::io::Result<()> {
    let mut local_file = File::create("hello.txt")?;
    saye_hello(&mut local_file)?;

    let mut bytes = vec![];
    saye_hello(&mut bytes)?;
    assert_eq!(bytes, b"Hello world");
    println!("{}", std::str::from_utf8(&bytes).unwrap());

    let a = [1.0, 2.0, 3.0];
    let b = [4.0, 5.0, 6.0];
    let result = dot_product(a, b);
    println!("{}", result);

    let c = '🦀';
    println!("{}", c.is_emoji());

    Ok(())
}
