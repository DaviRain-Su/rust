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
