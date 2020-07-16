# 泛型、trait 和生命周期

每一个编程语言都有高效的处理重复概念的工具，在rust中其中之一就是泛型。泛型是具体类型或其他属性的抽象替代。

我们可以表达泛型的属性，比如他们的行为或如何与其他泛型相关联，而不需要编写和编译代码时知道他们在这里实际上代表什么。

同理为了编写一份可以用于多种具体值的代码，函数并不知道其参数为何值，这时就可以让函数获取泛型而不是像i32或String这样的具体值。

提取函数以减少代码重复的机制，使用一个只在参数类型上不同的泛型函数来实现相同的功能。

trait可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型。

生命周期，它是一类允许我们向编译器提供引用相互关联的泛型，
Rust的生命周期功能允许在很多场景下借用值同时仍然使编译器能够检查这些引用的有效性。

## 提取函数来减少重复

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);
        assert(largest, 100);
    }


    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        
        println!("The largest number is {}", largest);

        assert(largest, 100);
    }

为了消除重复，可以创建一层抽象， 在这个例子中将表现为一个获取任意整型列表作为参数并对其进行处理的函数。
这将增加代码的简洁性并让我们将表达和推导寻找列表中最大值这个概念与使用这个概念的特定位置相互独立。

    fn largest(list: &[i32]) -> i32 {// &[i32] 传递给函数的任何具体的i32的slice
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {}", largest);

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let result = largest(&number_list);
        println!("The largest number is {}", largest);

    }
## 泛型数据类型

泛型用于通常我们放置类型的位置，比如函数签名或结构体，允许我们创建可以代替许多具体数据类型的结构体的定义。

### 在函数定义中使用泛型

定义函数时可以在函数签名的参数数据类型和返回值中使用泛型。

    fn largest_i32(list: &i32) {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);

        println!("The largest numner is {}", result);

        let char_list = vec!['y', 'm', 'a','q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }


为了参数化要定义的函数的签名中的类型，需要像函数的值参数起名一样为类型参数起一个名字。

任何标识符都可以作为类型参数名，选择T是因为rust的类型命名规范是骆驼命名法。

泛型类型参数的规范倾向于简短。

在需要在函数体中使用一个参数时，必须在函数签名中声明这个参数以便编译器能知道函数体中这个名称的意义。

当函数签名中使用一个类型参数时，必须在使用它之前就声明它，类型参数声明位于函数名称和参数列表中间的间括号中。

    fn largest<T>(list : &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);

        println!("The largest numner is {}", result);

        let char_list = vec!['y', 'm', 'a','q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }

## 结构体定义中的泛型

可以使用<> 语法来定义拥有一个或多个泛型参数类型字段的结构体。

    struct Point<T> {
        x: T, 
        y: T,
    }

    fn main() {
        let integer = Point { x: 5, y: 10};
        let float = Point { x: 1.0, y: 4.0};
    }
必须在结构体名称后面的间括号中声明泛型参数的名称，接着在结构体定义中可以指定具体数据类型的位置使用泛型类型。

如果想要定义x和y可以有不同类型且让然是泛型的Point结构体，可以使用多个泛型类型参数。

    struct Point<T, U>{
        x: T, 
        y: U,
    }

    fn main() {
        let both_integer = Point{ x: 5, y: 10};
        let both_float = Point{x: 1.0, y: 4.0};
        let integer_and_float = Point{ x: 5, y: 4.0};
    }

如果你处于一个需要很多泛型类型的位置，可能需要重新组织代码并分割一些更小部分的信号。

## 枚举定义中的泛型数据类型

枚举也可以在其成员中存放泛型数据类型。
    
    enum Option<T> {
         Some(T),
         None,
    }

一个可能的值是比一个具体类型的值更抽象的概念，rust允许我们不引入重复代码就能表现出抽象的概念。

    enum Result<T, E> {
        Ok(T), 
        Err(E),
    }

当发现代码中有很多只有存放值的类型有所不同的结构体和枚举体定义时，应该像之前函数定义中那样引入泛型类型来减少重复代码。

## 方法定义中的枚举数据类型

    struct Point<T> {
        x: T, 
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point{x: 5, y: 10};

        println!("p.x = {}", p.x());
    }

注意必须在impl后面声明T， 这样就可以在Point<T> 上实现的方法中使用它来。在impl之后声明泛型T， 这样rust就知道Point的间括号的类型是泛型而不是具体类型。

可以选择为Point<i32> 实例实现方法，而不是为泛型Point实例。

    struct Point<T> {
        x: T, 
        y: T,
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

这意味着Point<f32> 类型会有一个方法distance_from_origin，而其他T不是f32类型的Point<T>实例则没有定义此方法。这个方法计算点实例与另一个坐标之间的距离，它使用来只能用于浮点型的数据运算符。

 
结构体中的泛型参数并不总是于结构体方法签名中使用的泛型是同一种类型。

    struct Point<T, U> {
        x: T, 
        y: U,
    }

    impl<T, U> Point<T, U> {
        fn mixup<V, W> (self, other: Point<V, W>) {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10.4};
        let p2 = Point { x: "hello", y: 'c'};

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }


注意泛型参数 T和U声明于impl之后，因为他们于结构体定义相对应，而泛型参数V和W声明于mixup之后，因为他们只是相对于方法本身的。

## 泛型代码的性能

Rust实现泛型的方式意味着你的代码使用泛型类型参数相比指定具体类型并没有任何速度上的损失。

rust通过在编译时进行泛型代码的单态化来保证效率，单态化是一个将泛型代码转变为实际放入的具体类型的特定代码的过程。

编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码。

可以使用泛型来编写不重复的代码，而rust将会为每一个实例编译其特定的代码，这意味着使用泛型时没有运行时开销，当代码运行，它的执行效率就跟好像手写每个具体定义的重复代码一样，这个单态化的过程正是rust泛型在运行时其高效的原因。

# trait： 定义共享的行为

trait允许进行另一种抽象，可以抽象类型所通用的行为。trait告诉rust编译器某个特定类型拥有可能与其他类型共享的功能。在使用泛型类型参数的场景中，可以使用trait bounds在编译时指定泛型可以是任何实现来某个trait的类型，并由此在这个场景下拥有我们希望的功能。

注意trait类似与其他语言中常被称为接口的功能。


## 定义trait

一个类型的行为由其可供调用的方法构，如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。

trait定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的的所必需的行为集合。

    pub trait Summarizable {
        fn summary(&self) -> String;
    }

使用trait关键子来声明一个trait，后面是trait的名字。

在大括号中声明描述实现来这个trait的类型所需要的行为的方法签名。 在这个例子是fn summary(&self) -> String .  
在方法签名后跟分号，而不是在打括号中提供实现，接着每一个实现这个trait的类型都需要提供其自定义行为的方法体，编译器也会确保任何实现Summarizable trait的类型都拥有与这个签名的定义完全一致的summary方法。

trait体中可以有多个方法，一行一个方法签名且都以分号结尾。

## 为类型实现trait

    pub trait Summarizable {
        fn summary(&self) -> String;
    }

    pub struct NewArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summarizable for NewArticle {
        fn summary(&self) -> String {
            format!("{}, by {} ({})", self.headling, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username : String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

**在类型上实现trait类似于实现与trait无关的方法。**

在impl块中 trait定义中的方法签名，不过不再后跟分号，而是需要打括号中编写函数体来为特定类型实现trait方法所拥有的行为。

一旦实现来trait，就可以用与NewArticle和Tweet实例的非trait方法一样的方式调用trait方法。

    let tweet = Tweet {
        username : String::from("horse_ebook"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

如果这个lib.rs 是对应aggregator crate的，而别人想利用我们crate的功能外加其WeatherForecast结构体实现Summarizable trait，在实现Summarizable trait之前他们首先就需要将其导入其作用域中。



    extern crate aggregator;

    use aggregator::Summarizable;

    struct WeatherForecast {
        high_temp: f64,
        low_temp: f64,
        chance_of_precipitation: f64,
    }

    impl Summarizable for WeatherForecast {
        fn summary(&self) -> String {
            format!("The hight will be {}, and the low will be {}, The chance of precipitation is {}%.", self.hight_temp, self.low_temp, self.chance_of_precipitation)
        }
    }

trait实现的一个需要注意的限制是： 只能在trait或对应类型位于我们crate本地的时候为其实现trait。换句话说，不允许对外部类型实现外部trait。

这个称为孤儿原则，


## 默认实现

有时为trait中的某些或全部方法提供默认的行为，而不是在每个类型的每个实现中都定义自己的行为很有用。

这样党委某个特定类型实现trait时，可以选择保留或重载每个方法的默认行为。

    pub trait Summarizable {
        fn summary(&self) -> String {
            String::from("(Read more...)")
        }
    }

如果像对NewArticle实例使用这个默认实现，而不是像那样定义一个字节的实现，则可以指定一个空的impl块

    impl Summarizable for NewArticle {}

即便选择不再直接为New Article 定义summary 方法来，因为summary 方法有一个默认实现而且NewArticle被指定为实现来Summarizable trait, 我们仍然可以对NewArticle的实例调用summary 方法

    let article = NewArticle {
        headline : String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh,PA, USA"),
        author: String::from("Iceburgh"),
        contenct: String:;from("The Pittsburgh Penguins once again are the best hockey theam in the NHL."),
    };

    println!("New article avilable !{}", article.summary());

默认实现允许调用相同trait中的其他方法，哪怕这个方法没有默认实现，通过这种方法，trait可以实现很多有用的功能而只需实现一小部分特定内容。

可以选择让Summarizable trait 也拥有一个要求实现的author_summary 方法，接着summary 方法则提供默认实现并调用author_summary 方法；

    pub trait Summarizable {
        fn author_summary (&self) -> String;

        fn summary(&self) -> String {
            format!("(Read more from {} ...)", self.author_summary)
        }
    }
为来使用这个版本的Summarizable ，只需在实现trait时定义author_summary 即可：

    impl Summarizable for Tweet {
        fn author_summary(&self) -> String {
            format!("@{}", self.username)
        }
    }

一旦定义来author_summary ,既可以对Tweet结构体的实例调用summary， 而summary的默认实现会调用我们提供的author_summary定义

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: Stirng::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());

注意在重载过的实现中调用默认实现是不可能的


## trait Bounds

可以对泛型类型参数使用trait，可以限制泛型不再适用于任何类型，编译器会确保其被限制为那些实现来特定trait的类型，由此泛型就会拥有我们希望其类型所拥有的功能，这被称为指定泛型的trait bounds。


定义一个函数notify来调用summary方法，它拥有一个泛型类型T的参数item，为来能够在item上调用summary而不出现错误，可以在T上使用trait bounds来指定item必须实现来summaruzable的类型。

    pub fn notify<T: Summarizable>(item : T){
        println!("Breaking news! {}", item.summary());
    }

trait bouds连同泛型类型参数声明一同出现，位于间括号后面。
使用任何其他类型，String 来调用notify的代码将不能编译，因为这些类型没有实现Summarizable。

可以通过 + 来为泛型指定多个trait bounds，如果我们需要能够在函数使用T类型的显示格式的同时也能使用summary方法，则可以使用trait bounds T : Summarizable + Display .这意味者 T 可以是任何实现 Summarizable 和 Display的类型。


对于拥有多个泛型类型参数的函数， 每一个泛型都可以有自己的trait bounds。

    fn some_function<T, U>(t: T, u: U) -> i32 
        where T: Display + Clone,
            U: Clone + Debug
    {

    }

## 使用trait bounds 来修复 largest 函数


    fn largest<T: ParticalOrd> (list: &[T]) -> T {}

    fn largest<T: ParticalOrd + Copy> (list: &[T]) -> {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item ;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

## 使用trait bound有条件的实现方法

通过使用带有trait bound的泛型impl块，可以有条件的只为实现来特定trait的类型方法。

例如 类型Pair<T>总是实现来new方法，不过只有Pair<T>内部的T类型实现来PartialOrd trait 来允许比较和Display trait来启用打印，才会沙欣cmp_display

    use std::fmt::Display;

    struct Pair<T> {
        x: T, 
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {
                x, 
                y,
            }
        }
    }

    impl<T: Display + PartialOrd> {
        fn cmp_display(&self){
            if self.x > self.y {
                println!("The largest member is x = {}", self.x);
            }else{
                println!("The largest largest member is y = {}", self.y);
            }
        }
    }

也可以对任何实现来特定trait的类型有条件的实现trait。对于任何满足特定trait bounds的类型实现trait被称为 blanket implementations, 他们被广泛用于rust标准库。

标准库为任何实现来Display trait的类型实现来ToString trait，

    impl<T: Display> ToString for T {
        ...
    }

因为标准库有来blanket implementation ，我们可以对任何实现来Display trait的类型调用由ToString定义的to_string 方法。

    let s = 2.to_string();

trait和trait bound 让我们使用泛型类型参数来减少重复，并仍然能够向编译器明确指定泛型类型需要拥有那些行为，因为我们向编译器提供来trait bound信息，它就可以检查代码中所用到的具体类型是否提供来正确的行为，在动态类型语言中，如果我们尝试调用一个类型并没有实现的方法，会在运行时出现错误。Rust将这些错误移动到来编译时，甚至在代码能够运行之前就强迫我们修复错误。另外我们也无需编写运行时检查行为的代码，因为在编译时就已经检查过来，这样相比其他那些不愿放弃泛型灵活性的语言有更好的性能。


生命周期则有助于确保引用在我们需要他们的时候一直有效。


## 生命周期与引用有效性

Rust中的每一个引用都有其生命周期，也就是引用保持有效的作用域。
大部分时候生命周期是隐含并可以推断的，正如大部分时候类型也是可以推断的一样。
类似与当因为有多种可能类型的时候必须注明类型，也会出现引用的生命周期以一些不同方式相关联的情况，所以rust需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对有效的。

## 生命周期避免来悬垂引用

生命周期的主要目标是避免悬垂引用，他会导致程序引用来非预期引用的数据。

    {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }

未初始化的变量不能被使用

      