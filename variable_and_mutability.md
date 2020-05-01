# 变量和可变性

变量默认是不可变的，这说明了rust安全和简单并发的优势。

当变量不可变时，意味着一旦值被绑定上一个名称，你就不能改变这个值。

    fn main(){
        let x = 5;// first assignment to 'x'
        println!("The value of x is :{}", x);
        x = 6;
        // cannot assign twice to immutable variable
        println!("The value of x is :{}", x);
    }

不能对不可变变量二次赋值，cannot assign twice to immutable variable x

如果代码的一部分假设一个值永远也不会改变，而另一部分代码改变

了它，第一部分代码就有可能以不可预料的方式运行。

rust编译器保障，如果声明一个值不会变，他就真的不会变，这意味着

当阅读和编写代码时，不需要追踪一个值如何以及哪里可能会被

改变，从而使代码易于推导。

变量默认不可变，可以通过在变量名之前加mut来使其可变。

这也向读者表明了其他代码将会改变这个变量的意图。

    fn main() {
        let mut x = 5;
        println!("The value of x is :{}", x);
        x = 6;
        println!("The value of x is :{}", x);
    }

使用大型数据结构时，适当使用可变变量，可能比复制和返回新

分配的实例更快。

对于较小的数据结构，总是创建新实例，采用更偏向函数式的风格

编程，可能会使代码更易理解，为可读性而遭受性能惩罚或许值得。

## 变量和常量的区别

类似于不可变变量，常量也是绑定到一个名称的不允许改变的值，

不过常量与变量是有区别的：

1. 不允许对常量使用mut，常量不光默认不能变，它总是不能变。
2. 声明常量使用const关键字而不是let，并且必须注明值的类型。
3. 常量可以在任何作用域声明，包括全局作用域，这在一个值需要被很多部分的代码用到时很有用。
4. 常量只能用于常量表达式，而不能作为函数调用的结果，或者其他只在运行时计算的值。
5. 在声明它的作用域之中，常量在整个程序生命周期中都有效，这
使得常量可以作为多处代码使用的全局范围的值。

将用于整个程序的硬编码的值声明为常量对后来的维护者了解值的意义
很有帮助，同时将硬编码的值汇总于一处，也能为将来修改提供方便。

## 隐藏

    fn main()
    {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("The value x is :{}", x);
    }


这与将变量声明为mut是有区别的，因为除非再次使用let关键字，
不小心尝试对变量重新赋值会导致编译时错误。
可以用这个值进行一些计算，不过计算完之后变量仍然是不变的。

mut与隐藏的另一个区别是，当再次使用let时，实际上创建了一个新
变量，我们可以改变值的类型，从而复用这个名字。

## 数据类型

标量和复合

Rust是静态类型语言，也就是说在编译时就必须知道所有变量的类型。

标量类型

标量类型代表一个单独的值，rust有四种基本的标量类型，整型，
浮点型，布尔类型和字符类型。


## 复合类型

复合类型可以将多个其他类型的值组合成一个类型，
rust有两个原生的复合类型，元组和数组。

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。

使用一个括号中的逗号分隔的值列表来创建一个元组，元组中的每一个
位置都有一个类型，而这些不同值的类型也不必相同。

    fn main(){
        let tup : (i32, f64, u8) = (500, 6.4, 1);
    }

tup变量绑定了整个元组，因为元组被认为是一个单独的复合元素。
为了从元组中获取单个值，可以使用模式匹配来解构元组。
    
    fn main(){
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;//模式解构

        println!("The value of y is :{}", y);
    }

除了使用模式匹配解构之外，也可以使用点号后跟值的索引来直接访问他们。

    fn main() {\
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
    }

## 函数

rust代码使用snake case作为函数和变量名称的规范风格。

在snake case中，所有字母都是小写并使用下划线分割单词。


    fn main(){
        println!("Hello world!");

        another_function();
    }

    fn another_function(){
        println!("Another function!");
    }

可以使用定义过的函数名跟括号来调用任意函数。

注意， 源码中another_function在main函数之后被定义，也可以在其
之前定义，rust不关心函数定义何处，只要他们被定义了。

函数也可以被定义为 拥有参数，他们是作为函数签名一部分的特殊变量，
当函数拥有参数时，可以为这些参数提供具体的值。
技术上讲，这些具体值被称为参数，不过通常在函数定义中的变量
和调用函数时传递的具体值都可以用parameter和argument而不加区别。

在函数签名中，必须声明每个参数的类型，这是rust设计中一个经过
慎重考虑的决定，要求在函数定义中提供类型注解意味着编译器不需要
在别的地方要求你注明类型就知道你的意图。

## 函数体

函数体由一系列的语句和一个可选的表达式构成。
因为rust是一个基于表达式的语言，这是一个需要理解的，不同于其他语言重要的区别

## 语句与表达式

语句是执行一些操作但不返回值的指令。
表达式计算并产生一个值。

使用let关键字创建变量并绑定一个值是一个语句。
函数定义也是语句，
语句并不返回值，因此不能把let语句赋值给另一个变量。

表达式计算出一些值，而且他们组成了其余大部分你将会编写的rust代码
表达式可以是语句的一部分
函数调用是一个表达式，
宏调用是一个表达式，
用来创建新作用域的大括号也是一个表达式。

表达式并不包含结尾的分号，如果在表达式的结尾加上分号，他就
变成了语句，这也就使其不返回一个值。


## 在let语句中使用if

因为if是一个表达式，所以可以在let语句的右侧使用它。

# 所有权

所有权系统是rust最特殊的部分，令rust无需垃圾回收即可保障内存安全。

借用，slice，rust如何在内存中布局数据

Rust，内存被一个所有权系统管理，它拥有一系列的规则试编译器在编译时进行检查。然和所有权系统的功能，都不会导致运行时开销。

 
    所有权的规则：
    1. rust中每一个值都有一个称为其所有者的变量。
    2. 值有且只能有一个所有者。
    3.当所有者变量离开作用域，这个值将被丢弃。

## 变量作用域

    作用域是一个项在程序中有效的范围。

    { // s is not vaild here , it's not yet declared
        let s = "hello"; // s is vaild from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer 
    

String 类型

    let s = String::from("Hello);

:: 运算符允许将特定的from函数置于string类型的命名空间下而不需要使用类似
string_from这样的名字。

## 内存与分配

对于字符串字面值，我们在编译时就知道其内容所以它直接被硬编码进最终的可执行文件中，
这使得字符串字面值快速且高效。这些属性都只来源于其不可变性。



Rust采取内存回收的策略，内存在拥有它的变量离开作用域后被自动释放。

        {
            let s = String::from("hello);// s is vaild from this point forward 

            // do stufff with s

        }// this scope is now over, and s is no longer vaild


当s离开作用域的时候，当变量离开作用域，rust为其调用一个特殊的函数，这个函数叫做drop， 在这里string的作者可以放置释放内存的代码，Rust在结尾 } 处自动调用drop。


Rust永远不会自动创建数据的 深拷贝，因此，任何自动的复制可以被认为对运行时性能影响较小。

Rust有一个叫做Copy trait的特殊注解，可以用字啊类型整型这样的存储在栈上的类型。如果一个类型拥有Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。

Rust不允许自身或其任何部分实现了Drop trait的类类型使用 Copy trait。 如果对其值离开作用域时需要特殊处理的类型使用Copy注解，将会出现一个编译时错误。

通过一用的简单规则，任何简单标量值的组合可以Copy的，任何需要分配内存，
或者本身就是某种形式资源的类型不会是Copy的。

- 所有整数类型， u32
- 布尔类型， bool，
- 所有浮点数类型，
- 元组，当且仅当其包含的类型也都是Copy的时候，（i32，i32）是copy的，不过（i32，strint）就不是。

## 所有权与函数

将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。


    fn main() {
        let s = String::from("Hello");// s comes into scope.

        takes_owership(s);// s's value moves into the function ...
                            // .. and so is no longer vaild here.
        
        let x = 5; // x comes into scope.

        makes_copy(x); // x would move into the function,
                        // but i32 is Copy, so it's okay to still
                        // use x afterward

    }// Here, x goes out of scope, then s. But since s's value was movec, nothing special happens.

    fn takes_ownership(some_string: String){// some_string comes into scope.
        println!("{}", some_string);
    }// Here, some_stirng goes out of scope and 'drop' is called, The backing memory is freed.

    fn makes_copy(some_integer: i32){ // some_integer comes into scope.
        println!("{}", some_integer);
    }// Here, some_integer goes out of scope, Nothing speciall happens.


## 返回值与作用域

返回值也可以转移作用域。

    fn main() {
        let s1 = gives_ownership();// gives_ownership moves its return value into s1;

        let s2 = String::from("hello");// s2 comes into scope.

        let s3 = takes_and_gives_back(s2);// s2 is moved into takes_and_gives_back, which also moves its return value into s3.
    
    }// Here, s3 goes out of scope and is dropped. s2 goes out of scope but was moved, so nothing happens. s1 goes out of scope and is dropped.

    fn gives_ownership() -> String{
        // gives_ownership will move its return value into the function that calls it.

        let some_string = String::from("Hello");// some_string comes into scope.

        some_string // some_string is returned and moves out to the called function.

    }

    // takes_and_gives_back will take a String and return one.
    fn takes_and_gives_back(a_string: String) -> String {// a_string comes into scope
        a_string // a_string is returned and moves out to the calling function
    }

变量的所有权总是遵循相同的模式，将值赋值给另一个变量时移动它。
当持有堆中数据值的变量离开作用域时，其值将通过drop被清理掉，除非数据被移动为另一变量所有。

在每一个函数中都获取并接着返回所有权可能有些冗余。如果想要函数中使用一个值但不获取所有权该怎么办？如果我们还要接着使用它的话，每次都传递出去再传递回来就有点烦人，
另外我们也可能想要返回函数体产生的任何数据（不止一个）。

    fn main() {
        let s1 = Stirng::from("Hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{}' is '{}'., s2, s1);
    }

    fn calculate_length(s:String) -> (String, usize){
        let length = s.len();// len() return the length of a String.

        (s, length)
    }


# 引用与借用

下面定义一个新的calculate_length函数，它以一个对象的引用作为参数而不是获取值的所有权；

    fn main() {

        let s1 = Stirng::from("Hello");

        len len = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

&符号就是引用，允许你使用值但不获取其所有权。

注意： 与使用&引用相对的操作是解引用，它使用解引用运算符，*。

&s1 语法允许我们创建一个指向值s1的引用，但是并不拥有它，因为并不拥有这个值，当引用离开作用域时其指向的值也不会被丢弃。

函数签名使用了&来表明参数s的类型是一个引用，

    fn calculate_length(s: &String) -> usize {// s is a reference to a string
        s.len()
    }// Here， s goes out of scope. But because it does not have owership of what it refer to , nothing happens.

变量s有效的作用域与函数参数的作用域一样，不过当引用离开作用域后并不丢失它指向的数据，因为我们没有所有权。函数使用引用而不是实际值作为参数意味者无需返回值来交还所有权，因为就不曾拥有所有权。

将获取引用作为函数参数称为借用。

修改借用的变量，这是不行的。

    fn main() {

        let s = String::from("hello");

        change(&s);
    }

    fn change(some_stirng: &String){ // use &mut String here to make mutable
        some_string.push(", world");
        // cannt borrow as mutable
    }

和变量默认是不可变的，引用也是这样的，默认不允许修改引用的值。

## 可变引用

    fn main() {
        let mut s = String::from("hello");

        change(&mut s);
    }

    fn change(some_string: &mut String) {
        some_string.push_str(", world);
    }


特点， 可变引用的限制： 在特定作用域中的特定数据有且只有一个可变引用。

    let mut s = String::from("Hello");
    
    // cannt borrow s as mutable more than once at a time

    let r1 = &mut s;
        // first mutable borrow  occurs here
    let r2 = &mut s;
        // second mutable borrow occurs here

### 数据竞争 

数据竞争是一种特殊类型的竞争状态，它可由这三个行为造成：

1. 两个或更多指针同时访问同一数据
2.至少有一个这样的指针被用来写入数据。
3.不存在同步数据访问的机制。

数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust避免来这种情况的发生，因为它甚至不会编译存在数据竞争的代码。

可以使用打括号来创建一个新的作用域来允许拥有多个可变引用，只是不能同时拥有；

    let mut s = String::from("Hello");

    {
        let r1 = &mut s;
    
    }// r1 goes out of scope here, so we can make a new reference with no problems.

当结合可变和不可变引用时有一个类似的规则存在，

    // cannot borrrow s as mutable because it also borrowed as imutable

    let mut s = String::from("Hello");

    let r1 = &s; // no problem
        // immutable borrow occurs here
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
        // mutable borrow occurs here

不能在拥有不可变引用的同时拥有可变引用。
不可变引用用户可不希望在它的眼皮底下值突然就被改变了。

多个不可变引用是没有问题的因为没有那个只能读取数据的人有能力影响其他人读取到的数据。

这是rust编译器在提早指出一个潜在的bug在编译时而不是运行时并明确告诉你问题在那，而不是任由你去追踪为何有时数据并不是你想象中的那样。

## 悬垂引用

在存在指针的语言中，容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针，所谓悬垂指针是指向的内存可能已经被分配给其他持有者。

在rust中编译器确保引用永远不会编程悬垂状态：
当我们拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

    fn main() {
        let reference_to_nothing  = dangle();
    }

    fn dangle() -> &String { // s is a new String
        let s = String:: from("hello");

        &s // we return a reference to the String , s
    }// Here, s goes out of scope, and is dropped. Its memory goes away. Danger!

因为s是dangle函数内创建的，当dangle的代码执行完毕后，s将被释放。
尝试返回一个它的引用。这意味着这个引用会指向一个无效的String， Rust不允许我们这么做。

        fn no_dangle() -> String {
            let s = String::from("hello");

            s
        }

## 引用的规则


1. 在任意给定时间，只能拥有如下中的一个：
    - 一个可变引用
    - 任意数量的不可变引用
2.引用必须总是有效的


# Slices

没有所有权的数据类型是slice， 

Slice允许你引用集合中一段连续的元素序列，而不用引用整个集合。


问题： 编写一个获取一个字符串并返回它在其中找到的第一单词的函数。如果函数没有在字符串中找到一个空格，就意味着整个字符串是一个单词，所以整个字符串都应返回。

    fn first_word(s: &String) -> usize {
        let bytes = s.as_bytes(); // 将代码分解成小块，因为需要一个元素一个元素的检查String中的值是否为空格，需要用as_bytes方法将String转化为字节数组。 let bytes = s.as_bytes();

        // iter 方法在字节数组上创建一个迭代器

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }


iter方法返回集合中的每一个元素，而enumerate包装iter的结果并返回一个元组，其中每一个元素是元组的一部分，返回元组的第一个元素是索引，第二个元素是集合中元素的引用。

enumerate方法返回一个元组，可以使用模式来解构，所以在for循环中，我们指定来一个模式，其中i是元组中的索引而&item则是单个字节，因为从 .iter().enumerate()中获取来集合元素的引用，所以模式中使用来&。


    fn first_word(s, &String) -> usize {
        let bytes = s.as_bytes();

        for(i,&item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    fn main() {
        let mut s = String::from("Hello world");

        let world = first_world(&s); // word will get the value s.

        s.clear(); //This empties the String, making it equal to "";
        
        // word still has the value 5 here , but there's noe more string that we could meaningfully use the value 5 with, word is now totally invaild!
    }

在调用s.clear（） 之后 在使用Word与s状态就完全没有联系，所以， word仍然包含5.


## 字符床slice

字符床slice是string中的一部分值的引用，

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

这个类似于获取整个String的引用不过带有额外的[0..5]部分，不同于整个String的引用，这是一个包含String内部的一个位置和所需元素数量的引用。
start..end语法表示一个以start开头并一直持续到但不包含end的range。

使用一个由中括号中的[starting_index..ending_index]指定的range创建一个slice，其中starting_index是包含在slice的第一个位置，ending_index则是slice最后一个位置的后一个值。

在内部，slice的数据解构存储来开始位置和slice的长度，长度对应ending_index - stating_index的值，

对于Rust的 .. range 语法，如果想要从第一个索引开始， 可以不写两个点号之间的值。

    let s = String::from("Hello");

    let slice = &s[0..2];
    let slice = &s[..2];

如果slice包含String的最后一个字节，也可以舍弃尾部的数字。

    let s = String::from("Hello");

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..];

同时可以舍弃这两个值来获取一个整个字符串的slice，

    let s = String::from("hello");

    let len = s.len();

    let slice = &s[0..len];
    let slice = &s[..];

第二个版本

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

在调用first_world时，会返回一个单独的与底层数据相联系的值。这个值由一个slice开始位置的引用和slice中元素的数量组成

    fn main() {
        let mut s = String::from("hello world");

        let word = first_word(&s);
            // immutable borrow occurs here


        s.clear();//error!
            // mutable borrow occurs here
    }
    // immutable borrow ends here
// cannot borrow s as mutable because it is also borrowed as immutable 

当拥有某值的不可变引用时，就不能再获取一个可变引用，因为claer需要清空strint，它尝试获取一个可变引用。

## 字符串字面值就是slice

    let s = "hello world";

这里的s的类型就是&str， 它是一个指向二进制程序特定位置的slice。这也就是为什么字符串字面值是不可变的，&str是一个不可变引用。

    fn first_world(s: &str) -> &str {}

如果有一个字符串slice，可以直接传递它，如果有一个string，则可以传递整个String的slice。
定义一个获取字符串slice而不是字符串的引用的函数使我们的API更加通用并且不会丢失任何功能：

    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for(i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    fn main() {
        let my_string = String::from("Hello world");

        // first_word works on slice of String's
        let word = first_word(&my_string[..]);

        let my_string_literal = "hello world";

        //first_word works on silces of stirng literals
        let word = first_word(&my_string_literal[..]);

        // since string literal's are string silces already,
        // this worlks too, without the slice sytax!

        let word = first_work(my_string_literal);
    }


## 其他类型的slice

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

slice的类型是&[i32];

所有权，借用和slice这些概念是rust可以在编译时保障内存安全的关键。

