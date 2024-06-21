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

//根据错误信息显示,`Spliceable`特征并不是"对象安全"的,因此无法作为特征对象使用。错误信息提供了以下解释:
//
//1. `E0038: 特征 `Spliceable` 不能被转换为对象`:
//   - 对于一个特征来说,要成为"对象安全"的,需要允许构建虚拟方法表(vtable)来动态解析方法调用。
//   - 这里的问题是,`Spliceable`特征中的`splice`方法在参数和返回类型中引用了`Self`类型,这使得该特征无法成为对象安全的。
//
//2. `E0277: 类型为 `dyn Spliceable` 的值的大小无法在编译时确定`:
//   - 这个错误与上一个错误相关。由于`Spliceable`特征不是对象安全的,因此无法将其用作特征对象(`dyn Spliceable`),因为编译器无法在编译时确定特征对象的大小。
//
//该参考资料建议,为了使特征成为对象安全的,可以考虑将`splice`方法移到另一个特征中,正如帮助信息中提到的。
//
//总之,问题在于`Spliceable`特征由于`splice`方法的定义方式而不是对象安全的,这阻止了它作为特征对象的使用。建议的解决方案是重构该特征以使其成为对象安全的。

//pub trait Spliceable {
//  fn splice(&self, other: &Self) -> Self;
//}

//pub fn splice_anything(left: &dyn Spliceable, right: &dyn Spliceable) {
//    let combo = left.splice(right);
//    todo!("Implement this function")
//}

pub trait Visible {
    fn visible(&self) -> bool;
}

//pub trait Creature: Visible {
//    fn position(&self) -> (i32, i32);
//    fn facing(&self) -> f64;
//}

// NOTE: this is another way to define the Creature trait
// 通过将`Visible`特征作为超特征(supertrait)来定义`Creature`特征,可以确保`Creature`特征的实现也必须实现`Visible`特征。
// 子trait只是对Self类型限界的简写
pub trait Creature
where
    Self: Visible,
{
    fn position(&self) -> (i32, i32);
    fn facing(&self) -> f64;
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
