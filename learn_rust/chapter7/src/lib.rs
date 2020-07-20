pub mod new_struct {
///# 结构化编程
/// 
/// Rust提供了结构体、枚举体和trait来支撑程序结构
/// 
/// ## 面向对象编程
/// 
/// Rust并不符合标准的面向对象语言的定义。Rust既不存在对象的概念，
/// 也没有父子继承的概念，然而，rust却支持面向对象的封装。传统面向对象中的
/// 父子继承是为了实现代码复用和多态，而本质在类型系统概念中属于子类型多态，而Rust使用
/// trait和泛型提供的参数化多态就完全满足了这个需求。对于代码复用，Rust是通过泛型单态化
/// 和trait对象来避免代码重复，从而支持代码复用的，虽然相对于传统面向对象语言中的父子继承
/// 来说功能较弱，但Rust提供来功能强大的宏（包括macro和procedural macro）系统来帮助复用代码
/// ，甚至还可以使用一些设计模式来避免代码重复。Rust还实现了一种名为特化的功能来增强
/// 代码的高效复用。
/// 
/// 
/// Rust对面向对象编程风格的支持可以总结为以下几点
/// - 封装，Rust提供来结构体和枚举体来封装数据，并使用pub关键字定义其字段可见性；
/// 提供了impl关键字来实现数据的行为。
/// - 多态，通过trait和泛型以及枚举体来允许程序操作不同类型的值
/// - 代码复用，通过泛型，trait对象，宏，语法扩展，代码生成来设计模式
/// 
/// ### 结构体
/// 
/// 结构体和枚举体是Rust中最基本的两种复合类型。对于Rust类型系统来说，这两种
/// 复合类型实际上属于同一种概念，它们都属于代数数据类型。代数数据类型的概念来自函数式语言，
/// 仅通过这两种数据类型就可以构造出大部分的数据结构。
/// 
/// #### 代数数据类型🈯之积类型
/// 
///
/// 
///
/// Book的所有成员字段均为复制语义，所以在代码中输出book时，可以正常编译。
/// 
/// Rust中的结构体属于代数数据类型中的积类型。积类型来自范畴论的术语
/// 
///
#[derive(Debug, Clone)]
pub struct Book<'a> {
    name: &'a str, // as a value statement , so name have 'Copy' trait
    isbn: i32,
    version: i32,
}


pub fn print_book() {
    let book = Book {
        name: "Rust programming ", isbn: 20200719, version: 1
    };
    let book2 = Book {version : 2, ..book};// 因为Book中所有成员都是复制语义，所以在执行这一步后，
    //仍然可以输出，book. 结构体更新语法 '..', book的所有权并未被转移，结构体的更新语法，允许使用"..."语法来减少重复。
    println!("{:?}", book);
    println!("{:?}", book2);
}

#[derive(Debug, Clone)]
pub struct BookMove {
    // String是具有移动语义的字段，因此BookMove不允许实现Copy
    name: String,// not implement the 'Copy' trait, default is move 
    isbn: i32,
    version: i32,
    // Rus不允许包含String类型字段的结构体实现Copy，
    //因此代数类型有力的保障来复合数据类型的类型安全。
    //需要注意的是，结构体更新语法会转移字段的所有权。
}


pub fn print_book_move() {
    let book = BookMove {
        name: "Rust programming".to_string(), isbn: 202007090, version: 1
    };
    let _book2 = BookMove{ version: 2, ..book};
                // value moved here 
                // move occurs because 'book.name' has type 'std::string::String', which does not implement the 'Copy' trait
    // println!("s{:?}", book);
                    // ^ value borrowed here after partial move
    println!("{:?}", _book2)
}

/// #### 使用结构体进行面向对象风格编程
/// 
/// 在终端输出指定颜色的字符
/// 
/// 在终端显示带颜色的字符，需要使用ANSI转义序列， ANSI转义序列就是只形如ESC和【组合而成的字符序列，可以实现在屏幕上定位光标或改变输出字符颜色等功能，
/// 所以也被称为控制字符，被定义于ASCII ESC有三种表述方法
/// - 在shell中表示为\e
/// - 以ASCII十六进制表示为\x1B
/// - 以ASCII八进制表示为\033
/// \x1b[为前缀，表示这是一个ANSI控制序列的开始，用分号相隔的31;43属于颜色代码，31是前景色，代表红色，43为背景色，代表黄色，字母m为结束符。
/// 最后\x1b[0m结尾代表重置全部属性，表示一个ANSI控制序列的结束。
/// 
/// 整个ANSI序列中动态变化的只有两部分，颜色和原始文本，因此有来初步的实现步骤：
/// 


}
