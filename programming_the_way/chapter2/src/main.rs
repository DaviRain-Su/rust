//extern crate std;
//use std::prelude::v1::*;
fn main() {
    pub fn answer() -> () {
        let a = 40;
        let b = 2;
        assert_eq!(sum(a, b), 42);
        // println!("sum(a,b) --> {}", sum(a, b));
    }
    pub fn sum(a: i32, b: i32) -> i32{
        a + b
    }
    answer();

    bool();
}

/// # 临时值
/// 
/// Basic usage
/// 
/// ```
/// pub fn temp() -> i32{
///     return 1;
/// }
/// let x = &temp();
/// temp() = *x; // error[E0070]: invalid left-hand size expression
/// ```
pub fn temp() -> i32 {
    return 1;
}

/// # 不变与可变
///
/// Basic usage:
/// 
/// ```
/// pub fn immutable_and_mutable() {
///     let a = 1; //默认不可变
///     //a = 2; // immutable and error: cannot assign twice to immutable variable
///     let mut b = 2; // 使用mut关键字声明可变绑定
///     b = 3; //mutable
/// }
/// imutable_and_mutable();
/// ```
pub fn  imutable_and_mutable(){
    let _a = 1;
    // a = 2; // cannot assign twice to immutable variable
    let mut _b = 2;
    _b = 3; // mutable
}

/// # 所有权
/// 
/// Basic usage: 
/// 
/// ```
/// pub fn ownership(){
///     let place1 = "hello";
///     //  ^^ 位置表达式 ^^ 值表达式
///     // ^ 位置上下文  ^ 值上下文
///     let place2 = "hello".to_string();
///     let other = place1; // Copy 
///                 //^^   位置表达式出现在值上下文中
///     println("{:?}", place1); // place1 还可以继续使用
///     let other = palce2; // Move
///                 // ^^ 位置表达式出现在了值上下文中
///     println!("{:?}", place2) // place2 不能再被使用
/// }
/// ownership();
/// ```
pub fn ownership(){
    let place1 = "hello";
    let place2 = "hello".to_string();
    let _other = place1;
    println!("{:?}", place1);
    let _other = place2;
    // println!("{:?}", place2);// place2 value used here after move
}

///# 引用
/// 
/// Basic usage:
/// 
/// ```
/// pub fn reference() {
///     let a = [1, 2, 3];
///     let b = &a;
///     println!("{:p}", b); // 0x7ffcbc067704
///     let mut c = vec![1, 2, 3];
///     let d = &mut c;
///     d.push(4);
///     println!("{:?}",d);
///     let e = &42;
///     assert_eq!(42, *e);
/// }
/// reference();
/// ```
pub fn reference(){
    let a = [1, 2, 3];
    let b = &a;
    println!("{:p}", b);
    let mut c = vec![1, 2, 3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d); // [1, 2, 3, 4]
    let e = &42;
    assert_eq!(42, *e);
}

/// # 函数定义
/// 
/// Basic usage
/// 
/// ```
/// pub fn fizz_buzz(num: i32) -> String {
///     if num % 15 == 0 {
///         return "fizzbuzz".to_string();    
///     }else if num % 3 == 0 {
///         return "fizz",to_string();    
///     }else if num % 5 == 0 {
///         return "buzz".to_string();
///     }else {
///         return num.to_string();     
///     }
/// }
/// assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
/// assert_eq!(fizz_buzz(3), "fizz".to_string());
/// assert_eq!(fizz_buzz(5), "buzz".to_string());
/// assert_eq!(fizz_buzz(13), "13".to_string());
/// ```
pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();    
    }else if num % 3 == 0 {
        return "fizz".to_string();    
    }else if num % 5 == 0 {
        return "buzz".to_string();
    }else {
        return num.to_string();     
    }
}


/// # 词法作用域
/// 
/// Basic usage:
/// 
/// ```
/// pub fn lecical_scope {
///     let v = "hello world!";
///     assert_eq!(v, "hello world!");
///     let v = "hello Rsut!";
///     assert_eq!(v, "hello Rsut!");
///     {
///         let v = "hello World!";
///         assert_eq!(v, "hello World!");
///     }
///     assert_eq!(v, "hello Rsut!");
/// }
/// lexical_scope();
/// ```
pub fn lexical_scope() {
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    let v = "hello Rust";
    assert_eq!(v, "hello Rust");
    {
        let v = "hello Wolrd";
        assert_eq!(v, "hello World!");
    }
    assert_eq!(v, "hello Rust");
}

/// # 函数指针： 函数作为参数
/// 
/// Basic usage:
/// 
/// ```
/// pub fn math(op: fn(i32, i32) -> i32, a: i32, b:i32) -> i32 {
///     op(a, b)
/// }
/// fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
/// 
/// let a = 2;
/// let b = 3;
/// assert_eq!(math(sum, a, b), 5);
/// assert_eq!(math(product, a, b), 6);
/// ```
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn is_true() -> bool { true }
/// # 函数指针： 函数作为返回值
/// 
/// Basic usage: 
/// 
/// ```
/// fn is_true() -> bool { true }
/// fn true_maker() -> fn() -> bool { is_true }
/// assert_eq!(true_maker()(), true);
/// ```
pub fn true_maker() -> fn() -> bool { is_true }

/// # CTEE : const fn 
/// 
/// Basic usage:
/// 
/// ```
/// //#![feature(const_fn)]
/// const fn init_len() -> usize {
///     return 5;
/// }
/// [0; init_len()];
/// ```
pub const fn init_len() -> usize {
    return 5;
}

/// # 闭包
/// 
/// Basic usage:
/// 
/// ```
/// pub fn closure() {
///     let out = 42;
///     //  普通函数
///     fn add(i: i32, j: i32) -> i32 { i + j + out }
///     //定义类型标注的闭包
///     let closure_annotated = |i: i32, j:i32| -> i32 { i + j + out}
///     //如果没有类型标注则由编译器推断类型
///     let closure_inferred = |i, j| i + j + out;
///     let i = 1;
///     let j = 2;
///     assert_eq!(3, add(i,j));
///     assert_eq!(45, closure_annotated(i,j));
///     assert_eq!(45, closure_inferred(i,j));
/// }
/// closure();
/// ```
pub fn closure() {
    let out  = 42;
    // fn add(i: i32, j: i32) -> i32 { i + j + out}
    fn add(i: i32, j: i32) -> i32 { i + j}
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out};
    let closure_inferred = |i, j| i + j + out;
    let i = 1;
    let j = 2;
    assert_eq!(3, add(i, j));
    assert_eq!(45, closure_annotated(i,j));
    assert_eq!(45, closure_inferred(i,j));
}

/// # 闭包： 作为参数
/// 
/// Basic usage : 
///  
/// ```
/// pub fn closure_math<F: Fn() -> i32> (op: F) -> i32 {
///     op()
/// }
/// let a = 2;
/// let b = 3;
/// assert_eq!(closure_math(|| a + b), 5);
/// assert_eq!(closure_math(|| a * b), 6);
/// ```
pub fn closure_math<F: Fn() -> i32 > (op: F) -> i32 {
    op()
}

/// # 闭包： 作为返回值（动态分发）
/// 
/// Basic usage: 
/// 
/// ```
/// fn two_times() -> Box<Fn(i32) -> i32> {
///     let i = 2;
///     Box::new(move |j| i * j )
/// }
/// let result = two_times();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times() -> Box<Fn(i32) -> i32> {
    let i = 2;
    Box::new(move |j| j * i)
}

/// # 闭包； 作为返回值（动态分发）rust2018
/// 
/// Basic usage:
/// 
/// ```
/// fn two_times_dyn() -> Box<dyn Fn(i32)-> i32> {
///     let i = 2
///     Box::new(move |j| j * i)
/// }
/// let result = two_times_dyn();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times_dyn() -> Box<dyn Fn(i32) -> i32> {
    let i = 2;
    Box::new(move |j| i * j)
}

/// # 闭包： 作为返回值（静态分发）
/// 
/// Basic usage:
/// 
/// ```
/// fn two_times_impl() -> impl Fn(i32) -> i32 {
///     let i = 2;
///     move |j| j * i
/// }
/// let result = two_times_imple();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}

pub fn bool() {
    let x = true;
    let y: bool = false;
    let x = 5; // 变量遮蔽
    if x > 1 {
        println!("x is bigger than 1");
    }
    println!("x is {:?}", x); // x is 5
    // assert_eq!(x as i32, 1);// 5 != 1
    assert_eq!(y as i32, 0); // y == 0
}