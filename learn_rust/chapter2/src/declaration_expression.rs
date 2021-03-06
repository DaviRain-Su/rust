/// ## 语句与表达式
/// Rust中的语法可以分成两大类： 语句 statement和表达式 expression.
/// 语句是指要执行的一些操作和产生副作用的表达式。表达式主要用于计算求值。
/// 
/// 语句又分为两种：声明语句 Declaration statement 和表达式语句 Expression statement
/// - 声明语句， 用于声明各种语言项，包括声明变量、静态变量、常量、结构体、函数等，以及通过extern 和 use 关键字
/// 引入包和模块等。
/// - 表达式语句， 特值以分号结尾的表达式。此类表达式求值结果将会被舍弃，并总是返回单元类型(). 
/// 
/// Basic Usage: 
///```
/// // extern crate std; // 声明语句
/// // use std::prelude::v1::*; //声明语句
/// pub fn answer() -> () {
///     let a = 40; // 声明语句
///     let b = 2; // 声明语句
///     assert_eq!(sum(a, b), 42); //宏语句
/// }
/// 
/// pub fn sum(a: i32, b: i32) -> i32 {
///     a + b // 表达式
/// }
/// 
/// answer(); // 表达式语句
/// ```
/// 声明语句，它们不需要求值，只是用来引入标准库以及prelude模块。这里将其注释
/// ，是因为rust会为每个crate都自动引入标准库模块，除非使用#'\['no_std'\]'属性明确指定了不需要标准库
/// 
/// 函数answer没有输入参数，并且返回值为单元类型(), 单元类型拥有唯一的值，就是它本身，为了描述方便，将
/// 该值称为单元值。单元类型的概念来自OCaml，它表示“没有什么特殊的价值”。所以，这里将单元类型
/// 作为函数的返回值，就表示该函数无返回值。通常无返回值的函数默认不需要在函数签名中指定返回类型。
/// 
/// assert_eq!则是宏语句，它是Rust提供的断言，允许判断给定的两个表达式求值结果是否相同。
/// 像这种名字以叹号结尾，并且可以像函数一样被调用的语句，在Rust中叫做宏。
/// 
/// Rust编译器在解析代码的时候，如果碰到分号，就会继续往后面执行；如果碰到语句，则执行语句；
/// 如果碰到表达式，则会对表达式求值,如果分号后面什么都没有，就会补上单元值（）。
/// 
/// 当遇到函数的时候，会将函数体的花括号识别为块表达式 block expression . 块表达式是由
/// 一对花括号和一系列表达式组成的，它总是返回块中最后一个表达式的值。
/// 
/// 从这个角度来说，可以将rust看作一切皆表达式。由于分号后面什么都没有时自动补单元值（）的特点，我们
/// 可以将Rust中的语句看作计算结果均为（）的特殊表达式。而对于普通的表达式来说，则会的都正常的求值结果。
/// 
pub fn answer() -> () {
    let a = 40;
    let b = 2;
    assert_eq!(sum(a, b), 42);
    pub fn sum (a: i32, b: i32 ) -> i32 {
        a + b
    }
}

