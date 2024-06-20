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
