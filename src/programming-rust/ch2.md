# Chapter 2 Rust Guide

## 2.1 rustup & Cargo

使用rustup 安装rust。

- cargo 是Rust的构建系统和包管理器。
- rustc 是Rust的编译器。
- rustdoc 是Rust的文档生成器。

## 2.2 Cargo

使用Cargo创建一个新的Rust项目。

```shell
cargo new hello_world
```

下面这个是生成的文件结构

```bash
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

Cargo.toml 文件是一个配置文件，用于描述项目的元数据和依赖项。

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

[dependencies]
```

Cargo.lock 文件是一个自动生成的文件，用于跟踪项目的依赖项。

src/main.rs 是一个Rust源文件。

```rust
fn main() {
    println!("Hello, world!");
}
```

使用Cargo构建项目。

```shell
cargo build
```

使用Cargo运行项目。

```shell
cargo run
```

使用Cargo检查项目。

```shell
cargo check
```

使用Cargo测试项目。

```shell
cargo test
```

使用Cargo发布项目。

```shell
cargo publish
```

使用cargo clean清理项目。

```shell
cargo clean
```

## 2. 3 Summary

- cargo new 创建一个新的Rust项目。
- cargo build 构建项目。
- cargo run 运行项目。
- cargo check 检查项目。
- cargo test 测试项目。
- cargo publish 发布项目。
- cargo clean 清理项目。

## 2.4 Rust函数

Rust函数是一段代码，用于执行特定的任务。

```rust
// 函数定义使用fn关键字
// 函数名是gcd
// 参数是a和b
// 返回值是u32类型,使用 -> 符号
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        // 返回值
        // 这是一个if的表达式的true分支，直接返回一个表达式
        // a 是一个表达式
        a
    } else {
        // 这是一个if的表达式的false分支，直接返回一个表达式
        gcd(b, a % b)
    }
}
```

Rust中使用花括号{}包起来的代码块是一个作用域，都可以用作表达式。

```rust
{
    println!("evaluating cos x");
    x.cos()
}
```

## 2.5 编写测试和运行

Rust中的测试是一个函数，它使用test属性标记。`#[test]` 属性是一个测试函数的标记。

```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(100, 10), 10);
    assert_eq!(gcd(27, 9), 9);
}
```

使用cargo test命令运行测试。

```shell
cargo test
```

## 2.6 处理命令行参数

Rust中的main函数有一个参数，这个参数是一个字符串数组。

```rust
fn main() {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    // 打印命令行参数
    println!("{:?}", args);
}
```
通过命令行传入参数计算两个数的最大公约数

```rust
// 引入标准库的env模块，提供了与执行环境交互的函数
// args()函数返回一个迭代器，迭代器的元素是字符串
use std::env;
// 将标准库的FromStr trait引入作用域,这个trait定义了一个from_str方法
// 使用u64::from_str方法将字符串转换为u64类型
use std::str::FromStr;

// main函数没有返回值，因此这里省略了 -> ()
fn main() {

    // 定义一个可变的u64类型的数组，用于存储命令行参数
    // 类型是通过推断出来的Vec<u64>类型
    // numbers.push(u64::from_str(&arg).expect("error parsing argument")) 推断出来的类型是u64
    let mut numbers = Vec::new();

    // args()函数返回一个迭代器，迭代器的元素是字符串
    // skip(1)函数跳过第一个参数，第一个参数是程序的名称
    // for循环迭代器中的每一个元素
    // arg是迭代器中的每一个元素
    // u64::from_str(&arg)将字符串转换为u64类型
    // expect("error parsing argument")是一个错误处理函数，如果转换失败，打印错误信息
    // 将转换成功的u64类型的值添加到numbers数组中
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        // 打印错误信息 使用eprintln!宏
        eprintln!("Usage: gcd NUMBER ...");
        // 显示的调用exit函数，返回1，表示错误
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // 这里的&numbers[1..]是一个切片，切片是一个引用,引用是一个指向数据的指针
    // 这里仅仅是在借用数据，不会拷贝数据。
    // 这里的m是一个引用
    // 这里的*是解引用操作符
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!(
        "The greatest common divisor of the numbers {:?} is {}",
        numbers, d
    );
}
```
