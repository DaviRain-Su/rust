# 枚举和模式匹配

枚举，也被称为enums， 枚举允许你通过列举可能的值来定义一个类型。

首先，会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。

Option，它代表一个值要么是某个值要么什么都不是。

match表达式中用模式匹配，针对不同的枚举值编写相应要执行的代码。

if let 简洁方便处理代码中的枚举结构。

rust的枚举与F#， OCaml和haskell这样的函数式编程语言中的代数数据类型最为相似。


## 定义枚举

IP标准： IPv4 和 IPv6，

枚举值只可能是其中一个成员， IPv4和IPv6从根本上讲仍是IP地址，所以当代码在处理适用于任何类型的IP地址的场景时应该把他们当作相同的类型。

可以通过在代码中定义一个 IpAddrKind 枚举来表现这个概念并列出可能的IP地址类型，V4和V6。这被称为枚举的成员。

    enum IpAddrKind {
        V4,
        V6,
    }

### 枚举值

    enum IpAddrKind {
        V4, 
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

注意，枚举的成员位于其标识符的命名空间中，并使用两个冒号分开，

这么设计的益处就是 IpAddrKind::V4,和 IpAddrKind::V6都是IpAddrKind类型的。

    enum IpAddrKind {
        V4, 
        v6,
    }

    fn route(ip_type: UpAddrKind) { }

    route(IpAddrKind::v4);
    route(IpAddrKind::v6);
目前没有一个存储实际IP地址数据的方法； 只知道它是什么类型的，

    enum IpAddrKind {
        V4, 
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind : IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    }

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    }

我们使用了一个结构体来将kind和address打包在一起，现在枚举成员就与值相关联了。

可以使用一种更简单的方式来表达相同的概念，仅仅使用枚举并将数据直接放进每一个枚举成员而不是将枚举作为结构体的一部分。

IpAddr枚举的新定义表明来V4 和 V6成员都关联来String值：

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));

直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。

用枚举替代结构体还有另一个优势：每个成员可以处理不同类型和数量的数据。

Ipv4版本的IP地址总是含有四个值在0哈255之间的数字部分。

如果想要将V4地址存储为四个u8值而v6地址仍然表现为一个string，这就不能使用结构体了，

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

这些代码展示了使用枚举来存储两种不同IP地址的几种可能的选择，

事实上存储和编码IP地址太常见来以至于标准库提供来一个开箱即用的定义。

他将成员中的地址数据嵌入到了两个不同形式的结构体中，他们对于不同的成语的定义是不同的。

    struct Ipv4Addr {
        // details 
    }

    struct Ipv6Addr {
        // details
    }

    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

这些代码展示了可以将任意类型的数据放入枚举成员中：字符串，数字类型或者结构体。
甚至可以包含另一个枚举。

注意虽然标准库中包含了一个IpAddr的定义，仍然可以创建和使用我们自己的定义而不会育冲突，因为我们并没有将标准库中的定义引入作用域。

    enum Message {
        Quit, 
        Move { x: i32, y: i32},
        Write (String),
        ChangeColor(i32m i32, i32),
    }

这个枚举有四个含有不同类型的成员：
- Quit 没有关联任何数据
- Move 包含一个匿名结构体
- Write 包含单独一个String
- ChangeColor 包含三个i32

枚举不使用struct关键字并且所有成员被组合在一起位于Message下之外。

如下这些结构体可以包含与之前枚举成员中相同的数据：

    struct QuitMessage;//unit struct 
    struct MoveMessage{
        x: i32, 
        y: i32,
    }

    struct WriteMessage(String);// tuple Struct 
    struct ChangeColorMessage(i32, i32, i32);// tuple Struct 

如果使用不同的结构体，他们都有不同的类型，将不能轻易的定义一个获取任何这些信息类型的函数。 定义的Message枚举，他们是一个类型

结构体和枚举有一个相似点，可以使用impl来为结构体定义方法那样，也可以在枚举上
定义方法。

    enum Message {
        Quit, 
        Move { x: i32, y: i32},
        Write (String),
        ChangeColor(i32m i32, i32),
    }

    impl Message {
        fn call(&self){
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("Hello"));
    m.call();

方法体使用来self来获取调用方法的值， 创建来一个拥有类型Message::Write("hello")的变量m，而且当m.call()运行时call方法中的self的值。

## Option枚举和其相对空值的优势


Option 类型应用广泛因为它编码来一个非常普遍的场景，即一个值要么是某个值要么什么都不是。从类型系统的角度来表达这个概念就意味这编译器需要检查是否处理来所有应该处理的情况，这样就可以避免在其他编程语言中非常常见的bug。

空值尝试表达式的概念仍然是有意义的；空值是一个因为某种原因目前无效或缺失的值。

Rust没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举就是Option<T>, 而且定义于标准库，

    enum Option<T>{
        Some(T), 
        None,
    }

这意味这不需要显式的引入作用域。他的成员也是如此。可以不需要Option::前缀直接使用Some和None，


即便如此Option<T> 也仍然是常规的枚举，Some(T)和None仍是常规的枚举类型，Some(T)和None仍是Option<T>的成员。

    let som_number = Some(5);
    let some_string = Some("a String");

    let absent_number: Option<i32> = None;

如果使用None而不是Some，需要告诉Rust Option<T>是什么类型，因为编译器只通过None值无法推断出some变量保留的值的类型。

当有一个some值时，我们知道存在一个值，而这个值保存在some中，当有个None值时，在某种意义上它跟空值是相同的意义；并没有一个有效的值。

option<T> 为什么比空值还要好？

因为option<T>, 是不同的类型，编译器，不允许像一个被定义的有效的类型那样使用Option<T>.

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
        // no implementation for i8 + std::option::Option<i8>
    // the trait bound i8 : std::ops::Add<std::option::Option<i8>> is not statisfied

当在rust中拥有一个像i8这样类型的值时，编译器确保它总是有一个有效的值，可以自信使用而无需判空。

只有当使用Option<i8> 的时候需要担心可能没有一个值，而编译器会确保我们在使用值之前处理为空的情况。

也就是说，在对Option<T>进行T的运算之前必须将其转换为T。

通常能帮助我们捕获空值最常见的问题之一： 假设某值不为空但实际上为空的情况。

任何地方一个值不是Option<T>类型的话，可以安全的假设它的值不为空。

当有一个Option<T>的值时，如何从Some成员中取出T的值来使用它？Option<T>枚举拥有大量用于各种情况的方法。

为来 使用Option<T>值，需要编写处理每个成员的代码，我们想要一些代码只当拥有Some(T)值时运行，这些代码允许使用其中的T。也希望一些代码在None值时运行，这些代码并没有一个可用的T值。



# match控制流运算符

Rust有一个叫做match的极为强大的控制运算符，它允许我们将一个值与一系列的模式相比较并根据匹配的模式执行相应代码。

模式可由字面值、变量、通配符和许多其他内容组成。

match的力量源于模式的表现力以及编译器检查，它确保来所有可能的情况都得到处理。

        enum Coin{
            Penny, 
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

match 关键字后跟一个表达式，与if不同的是，if表达式必须返回一个布尔值，而这里match可以是任何类型的。

match的分支， 一个分支有两个部分：一个模式和一些代码。
=> 运算符将模式和将要运行的代码分开。

当match表达式执行时，它将结果按值的顺序与每一个分支的模式相比较，如果模式匹配来这个值，这个模式相关联的代码将被执行。如果模式并不匹配这个值，将继续执行下一个分支。

每个分支相关联的代码是一个表达式，而表达式的结果将作为整个match表达式的返回值。

如果想再分支中运行多行代码，可以使用打括号。

    enum Coin {
        Penny, 
        Nickel,
        Dime, 
        Quarter,
    }

    value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => {
                println!("Luck penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

## 绑定值的模式

 匹配分支的另一个有用的功能是可以绑定匹配的模式的部分值，也就是如何从枚举成员中提取值。
    
    #[derive(Debug)]
    enum UsState {
        Alabma,
        Alasks,
        // .... etc
    }

    enum Coin {
        Penny, 
        Nickel,
        Dime,
        Quarter(UsState),
    }

在匹配Coin::Quarter成员的分支的模式中增加了一个叫做state的变量，当匹配搭配Coin::Quarter时，变量state将会绑定25美分所对应的州的值。


    #[derive(Debug)]
    enum UsState {
        Alabma,
        Alasks,
        // .... etc
    }

    enum Coin {
        Penny, 
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }

## 匹配 Option<T>

使用Option<T> 时想要从Some中取出其内部的T值。

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

匹配是穷尽的

     fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }// pattern None not covered
    }
Rust中匹配是穷尽的；必须穷举到最后的可能性来使代码有效。特别在这个Option<T>的例子中，rust防止我们忘记明确的处理None的情况。

_通配符

Rust提供来一个模式用于不想列举出所有可能值的场景。

    let some_u8_value = 0u8;

    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

_ 模式会匹配到所有的值，通过将其放置于其他分支之后，_ 将会匹配到所有之前没有指定的可能的值。
（）就是 uint值， 所以 _ 的情况什么也不会发生。

对 _ 通配符之前没有列出的所有可能的值不做任何处理。

# if let 简单控制流

if let 语法让我们以一种不那么冗长的方式结合if 和 let， 来处理只匹配一个模式的值而忽略其他模式的情况。

    let some_u8_value = Some(0u8);
    match some_u8_value {
        some(3) => println!("three");
        _ => ();
    }

    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

可以在if let中包含一个else， else块中的代码与match表达式中的 _ 分支的代码相同。这样match表达式就等同于if else 和 else。




