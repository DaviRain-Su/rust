/// # 基本知识
/// 
/// 探讨函数， 变量和最基本的类型等基本知识。
/// 
/// ## 变量
/// 
/// 变量使用let关键字来声明
/// 在赋值时，Rust能99%的情况下推断变量的类型，如果不能，可以将类型添加到变量
/// 声明中。
/// 注意如果多次分配相同的变量名，这被称为变量隐藏， 可以改变变量的类型以实现对变量名的后续使用。
/// 
/// 变量名总是遵循蛇形命名法。
/// 
/// Basic Usage: 
/// ```
/// pub fn variable_type() {
///     let x = 13; //rust 自动推导类型
///     println!("{}", x);
///     
///     let x : f64 = 3.141595; // rust显示声明类型
///     println!("{}", x);
/// 
///     //rust也支持先声明后初始化，但很少这么做
///     let x;
///     x = 0;
///     println!("{}", x);
///     
/// }
/// ```
pub fn variable_type() {
    let x = 13;
    println!("{}", x);

    let x : f64 = 3.1415;
    println!("{}", x);

    let x;
    x = 0;
    println!("{}", x);
}

/// ##修改变量
/// 
/// Rust非常关心那些变量是可修改的。值分为两种类型：
/// - 可变的， 编译器允许变量进行写入和读取
/// - 不可变的，编译器只允许对变量进行读取
/// 
/// 可变值用mut关键字表示。
/// 
/// Basic Usage:
/// ```
/// pub fn change_variable_type() {
///     let mut x = 42;
///     println!("{}", x);
///     x = 13;
///     println!("{}", x);
/// }
/// 
pub fn change_variable_type() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
}

/// ## 基本类型
/// 
/// - 布尔型， bool 表示true或false
/// - 无符号整型， u8, u32, u64, u128表示正整数
/// - 有符号整型， i8, i32, i64, i128表示正负整数
/// - 指针大小的整数， usize, isize 表示内存中内容的索引和大小
/// - 浮点数， f32, f64
/// - 元组，（value, value, ...) 用于在栈上传递固定序列的值
/// - 数组， 在编译时已知的具有固定长度的相同元素的集合
/// - 切片， 在运行时已知长度的相同元素的集合
/// - str，在运行时已知长度的文本
/// 
/// 可以通过将类型加到数字的末尾来明确指定数字类型
/// 
/// Basic Usage: 
/// ```
/// pub fn basic_type() {
///     let x = 12; // default is i32
///     let a = 12u8;
///     let b = 4.3; // default is f64
///     let c = 4.3f32;
///     let bv = true;
///     let t = (13, false);
///     let sentence = "hello world!";
///     println!(
///         "{} {} {} {} {} {} {} {} ", 
///         x, a, b, c, bv, t.0, t.1, sentence
///     );
/// }
/// ```
pub fn basic_type() {
    let x = 12;
    let a = 12u8;
    let b = 4.3;
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world";
    println!("{} {} {} {} {} {} {} {}", x, a, b, c, bv, t.0, t.1, sentence);
}
/// ## 基本类型转换
/// 
/// 当涉及到数字类型时，Rust要求明确，一个人不能把“u8”用在
/// “u32”上而不出错。
/// 
/// 使用as关键字，rust使数字类型转换非常容易。
/// 
/// ```
/// pub fn as_type() {
///     let a = 13u8;
///     let b = 7u32;
///     let c = a as u32 + b;
///     println!("{}", c);
/// 
///     let t = true;
///     println!("{}", t as u8);
/// }
/// ```
pub fn as_type() {
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

/// ## 常量
/// 
/// 常量允许指定一个在代码中多次有效使用的公共值，常量在编译使直接使用它们
/// 的值替换标识符，而不是复制变量之类的值。
/// 
/// 不同于变量，常量必须始终具有显式类型
/// 
/// 常量名总是遵循全大写蛇形命名法
/// 
/// ```
/// pub fn const_type() {
///     const PI: f32 = 3.1415;
///     
///     println!("To make a apple {} from scratch, you must first create a universe.", 
///     PI
///     );
/// }
/// ```
pub fn const_type() {
    const PI : f32 = 3.1415;
    println!("To make a apple {} from scratch, you must first create a universe.", PI);
}

/// ## 数组
/// 
/// 数组是所有相同类型数据元素的固定长度集合。
/// 
/// 一个数组的数据类型是[T;N], 其中T是元素的类型，N是编译时已知的固定长度。
/// 
/// 可以使用'\['x\']'运算符检索单个元素，其中x是所需元素的usize索引（从0开始）
/// ```
/// pub fn array_type() {
///     let nums: [i32; 3] = [1,2,3];
///     println!("{:?}", nums);
///     println!("{}", nums[1]);
/// }
/// ```
pub fn array_type() {
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}

/// ## 函数
/// 
/// 函数有0个或者多个参数
/// 函数名总会遵循蛇形命名法
/// 
/// ```
/// pub fn add(x: i32, y: i32) -> i32 {
///     x + y
/// }
/// add(42, 23);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

/// ## 多个返回值
/// 
/// 函数可以通过元组来返回多个值
/// 
/// 元组可以通过它们的索引来获取
/// 
/// ```
/// pub fn swap(x: i32, y: i32) -> (i32, i32) {
///     (y, x)
/// }
/// 
/// let result = swap(123, 321);
/// println!("{} {}"m result.0, result.1);
/// 
/// let (a, b) = swap(result.0, result.1);
/// println!("{} {}", a, b);
/// ```
pub fn swap(x: i32, y: i32) -> (i32, i32){
    (y, x)
}

/// ## 返回空
///
/// 如果没有为函数指定返回类型，它将返回一个空的元组， 也称为单元。
/// 
/// 一个空的元组用（）表示
/// 
/// 明确的使用（） 是不常见的，作为函数返回值会经常出现。
/// 
/// ```
/// pub fn make_nothing() -> () {
///     ()
/// }
/// 
/// //default return ()
/// pub fn make_nothing2() {
/// 
/// }
/// 
/// let a = make_nothing();
/// let b = make_nothing2();
/// 
/// println!("The value of a : {:?}", a);
/// println!("The valur of b : {:?}", b);
/// ```

pub fn make_nothing() -> () {
    ()
}