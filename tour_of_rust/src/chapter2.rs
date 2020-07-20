/// # 基本控制流
/// 
/// 讨论rust中的基本控制流方法
/// 
/// ## if/else if / else
/// 
/// 条件判断没有括号 
///  常用的逻辑运算符：==, != , <, >, <=, >=, !, ||, &&
/// 
/// ```
/// pub fn control_flow(){
///     let x = 42;
///     if x < 42 {
///         println!("less than 42");
///     }else if x == 42 {
///         println!("is 42");
///     } else {
///         println!("greater than 42");
///     }
/// }
/// ```
/// 
pub fn control_flow() {
    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
}

/// ## 循环
/// 
/// break会退出循环
/// loop
/// 
/// ```
/// pub fn loop_flow() {
///     let mut x = 0;
///     loop {
///         x += 1;
///         if x == 42 {
///             break;
///         }
///     }
///     println!("{}", x);
/// }
/// ```
pub fn loop_flow() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);
}

/// ##  while
///
/// while允许你轻松地向循环中添加条件
/// 如果条件变为false, 循环退出。
/// 
/// ```
/// pub fn while_flow() {
///     let mut x = 0;
///     while x != 42 {
///         x += 1;
///     }
///     println!("x is {}", x);
/// }
/// ```
pub fn while_flow() {
    let mut x =  0;
    while x != 42  {
        x += 1;
    }
    println!("x is {}", x);
}

/// ## for
/// rust的for循环是一个强大的升级。他遍历来自计算机结果为迭代器的任何
/// 表达式的值，迭代器是什么？迭代器是一个你可以询问下一项是什么？直到没有
/// 其他项的对象。
/// 
/// .. 运算符创建一个可以生成包含起始数字、但不包含末尾数字的数字序列的迭代器。
/// ..= 运算符创建一个可以生成包含起始数字、且末尾数字的数字序列的迭代器
/// 
/// ```
/// pub fn for_flow() {
///     for x in 0..5 {
///         println!("The cur is {}", x);    
///     }    
/// 
///     for x in 0..=5 {
///         println!("The cur is {}",x);
///     }
/// }
/// ```
pub fn for_flow() {
    for x in 0..5 {
        println!("The cur is {}", x);
    }

    for x in 0..=5 {
        println!("The cur is {}", x);
    }
}

/// ## match 
/// Rust又一个非常有用的关键字，用于匹配值的所有可能条件，
/// 并在匹配为真时执行相应代码。 
/// 
/// match是穷尽性的，所以必须处理所有情况。
/// 
/// ```
/// pub fn match_flow () {
///     let x = 42;
///     
///     match x {
///         0 => {
///             pritnln!("found zero");
///         }
///         
///         1 | 2 => {
///             println!("found 1 or 2!");
///         }
///         
///         3..=9 => {
///             println!("found a number 3 to 9 inclusively");    
///         }
///         
///         matched_num @ 10..=100 => {
///             println!("found {} number between 10 to 100!", matched_num);
///         }
///         
///         _ => {
///             println!("found somethind else!");
///         }
///     }
/// }
/// ```
pub fn macro_flow() {
    let x = 42; 

    match x {
        0 => {
            println!("foubd zero!");
        }

        1 | 2 => {
            println!("found 1 or 2!");
        }

        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }

        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100 !", matched_num);
        }

        _ => {
            println!("found something selse !");
        }
    }
}

/// ## 从循环中返回值
/// loop可以中断以返回一个值
///     
/// ```
/// pub fn return_value_from_loop () {
///     let mut x = 0;
///     let v = loop {
///         x += 1;
///         if x == 13 {
///             break "found the 13";
///         }
///     };
///     println!("from loop: {}", v);
/// }
/// ```
pub fn return_value_from_loop() {

    let mut x = 10;
    let v = loop {
        x += 1;
        if x == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);
}

/// ## 从块表达式返回值
/// 
/// if, match， 函数， 以及作用域都有一种返回值的独特方式
/// 
/// 如果if, match , 函数或作用域块中的最后一条语句是不带;的表达式，
/// Rust将把它作为一个值从块中返回。 这是一种创建简洁逻辑的好方法，他返回一个可以放入新变量的值。
/// 
/// 注意， 他还允许if语句像简洁的三元表达式一样操作
/// 
/// ```
/// pub fn return_block_expression() -> i32 {
///     let x = 42;
///     
///     // rust three expression
///     let c = if x < 42 { -1 } else { 1 };
///     prinln!("from if : {}", v);
///     
///     let food = "hamburger";
///     let result = match food {
///         "hotdog" => "is hotdog",
///         //  注意，当它只是一个返回表达式时，大括号是可选的
///         _ => "is not hotdog",
///     }
///     println!("identifying food : {}", result);
/// 
///     let v = {
///         let a = 1;
///         let b = 2;
///         a + b
///     };
///     println!("from block : {}", v);
///     
///     v + 4
/// }
/// ```
pub fn return_block_expression() -> i32 {
    let x = 42;
    let v = if x <  42 { -1 } else { 1 };
    println!("from if {}", v);

    let food = "hambuger";
    let result = match food {
        "hotdog" => "is hotdog",
        _ => "is not hotdog",
    };
    println!("identifying food : {}", result);

    let v = {
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block {}", v);
    v + 4
}