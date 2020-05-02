# 使用结构体组织相关联的数据

struct 或者structre，是一个允许我们命名并将多个相关值包装进一个有意义的组合的自定义类型。

对比元组和结构体的异同，展示如何使用结构体，并讨论如何在结构体上定义方法和相关联函数来指定与结构体数据相关的行为。

## 定义并实例化结构体

结构体与元组类似，结构体的每一部分可以是不同类型，不同于元组，结构体需要命名各个部分数据以便能清楚的表明其值的意义。

由于有来这些名字使得结构体比元组更灵活；不需要依赖顺序来指定或访问实例中的值。

定义结构体，需要使用struct关键字为整个解构体提供一个名字。结构体的名字需要描述它所组合的数据的意义。在打括号中，定义一部分数据的名字，他们被称作字段，并定义字段类型。

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

一旦定义来结构体之后，为来使用它，通过为每个字段指定具体值来创建这个结构体的实例。

创建一个实例需要以结构体的名字开头，接着在打括号中使用key：value对的形式提供字段，其中key是字段的名字，value是需要存储在字段中的数据值。

实例中具体说明字段的顺序不需要和他们在结构体中声明的顺序一致。

换句话说，结构体的定义就像一个类型的通用模版，而实例则会在这个模版中放入特定数据来创建这个类型的值。

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        email : String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

为来从结构体体中获取特定的值，可以使用点号。

要是改变结构体中的值，如果结构体的实例是可变的，我们可以使用点号并为对应的字段赋值。

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User { // note the mut 
        email : String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

注意的是，整个实例必须是可变的，rust不允许只将特定字段标记为可变。

同其他任何表达式一样，可以在函数体的最后一个表达式构造一个结构体，从函数隐式的返回一个结构体的新实例。

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email: email, 
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

## 变量与字段同名时的字段初始化简写语法

字段初始化化简写语法

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            email,  // note
            username, // note 
            active: true,
            sign_in_count: 1,
        }
    }
build_user 函数使用来字段初始化简写语法， 因为email和username参数与结构体字段同名，在函数内创建来一个新的User结构体实例，将email和username参数设置为build_user  函数的参数，因为email字段与username参数有着相同的名称，则只需编写email而不是email：email。

## 使用结构体更新语法从其他对象创建对象

可以从老的对象创建新的对象，即复用大部分老对象的值并只改变一部分值，这可以通过架构体更新语法实现。

未使用结构体更新语法

     struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        sctive: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        usernamme: String::from("anotherusername"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

使用结构体更新语法 ，可以通过更少的代码达到相同的效果，
.. 语法指定老剩余未显示设置值的字段应有与给定实例对应字段相同的值。

    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        sctive: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user1
    };

注意这里的更新语法，只是作用在了具有Copy tarit特性的类型上。

## 使用没有命名字段的元组结构体来创建不同的类型

也可以定义与元组类似的结构体，称为元组结构体，有着结构体名称提供的含义，但是没有具体的字段名，只有字段的类型。

**元组结构体在你希望命名整个元组并使其他同样元组为不同类型时很有用**
这时像常规结构体那样为每个字段命名就显得冗余和形式化了。

定义元组结构体以struct关键字和结构体名开头后跟元组中的类型。

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

这里的black和origin值是不同的类型，因为他们是不同元组结构体的实例，
定义的每一个结构体有自己的类型，即使结构体中的字段有着相同的类型。

即，一个获取color类型的参数不能接收point作为参数，即使这两个类型都由三个i32值组成。

其他，元组结构体实例类似于元组：可以将其解构为单独的部分，也可以使用.后跟索引来访问单独的值。

## 没有任何字段的类单元结构体

定义一个没有任何字段的结构体，称为类单元， 因为他们类似于（），即unit类型，
类单元结构体常常在想要某个类型上实现trait但不需要在类型内存储数据的时候发挥作用。


## 一个使用结构体的示例程序

    fn main() {
        let width1 = 30;
        let height1 = 50;

        println!("The area of the rectangle is {} square pixels."
            area(width1, height1)
        );
    }

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

使用元组重构

	fn main() {
        let rect1 = (30, 50);

        println!("The area of the rectangle is {} square pixels.", area(rect1));
    }

    fn area(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }

使用结构体重构：

    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangel {width: 30, height: 50};
        
        println!("The area of the rectangle is {} square pixels.", area(&rect1));
    }

    fn area(rectangle: &Rectangle) -> u32 {//参数为Reactangle实例的不可变借用
        rectangle.width * rectangle.height
    }

通过派生trait增加实用功能

println！宏处理很多格式类型， {}默认告诉println使用称为Display的格式： 意味在提供给直接终端用户查看输出。

所有的基本类型都默认实现来Display

{} 中加入:? 指示符告诉println！我们想要使用叫做Debug的输出格式，

Debug是一个trait，允许我们在调试代码时以一种对开发者有帮助的方式打印出解构体。

易读可以使用，  {:?}替换println字符串中的{:?},


## 方法语法

方法于函数类似， 他们使用fn 关键字和名称声明，可以拥有参数和返回值，同时包含一段方法在某处被调用时会执行的代码。

不过与函数不同的是，因为他们在结构体的上下文中被定义（或者是枚举或者trait对象的上下文），并且他们第一个参数总是self，它代表调用该方法的结构体实例。

### 定义方法

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn areq(&self) -> u32 {
            self.width * self.height
        }
    }

    fn main() {
        let rect1 = Rectangel {width: 30, height: 50};
        
        println!("The area of the rectangle is {} square pixels.", rect1.area());
    }

为了使函数定义于Rectangle的上下文中，开始了一个impl块，

将函数移动到impl块中，并将签名中的第一个参数和函数中其他地方的对应参数改为了self。

然后在main汇总将我们先前调用area方法并传递rect1作为参数的地方，改成使用方法语法在Rectangle实例上调用area方法。方法语法获取一个实例并加上一个点号，后跟方法名、括号以及任何参数。

在area的签名中，开始使用&self来替代rectangle : &Rectangle,因为该方法位于imple Rectangle 上下文中所以rust知道self的类型是Rectangle。

注意的仍然需要在self前面加上&，就像&Rectangle一样，方法可以选择获取self的所有权，或者像我们一样不可变地借用self，或者可变地借用self，就跟其他别的参数一样。

这里选择&self跟在函数版本中使用&Rectangle出于同样的理由，并不想获取所有权，只希望能读取结构体中的数据，而不是写入。如果想要在方法中改变调用方法的实例，需要将第一个参数改为&mut self。通过仅仅使用self作为第一个参数来使方法获取实例的所有权是很少见的： 这种技术通常在当方法将self转换成别的实例的时候，这时我们要防止调用者在转换之后使用原始的实例。

使用方法替代函数，除了使用方法语法和不需要在每个函数签名中重复self类型之外，其主要好处在于组织性。我们将某个类型实例能做的所有事情一起都放入impl块中，而不是让将来的用户在我们的库中到处寻找Rectangle功能。


Rust中并没有一个与 -> 等效的运算符；rust中有一个叫做自动引用和解引用的功能，方法调用是rust中少数几个拥有这种行为的地方。

当使用object.something()调用方法时，rust会自动增加&， &mut或 * 以便使 object符合方法的签名。

    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance (&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared  + y_squared)
        }
    }

    let p1 = Point{ x: 0.0, y: 0.0};
    let p2 = Point{ x: 5.0, y: 6.5};

    p1.distance(&p2);
    (&p1).distance(&p2);

### 带有更多参数的方法

    fn main() {
        let rect1 = Rectangle { width: 30, height: 50};
        let rect2 = Rectangle { width: 10, height: 40};
        let rect3 = Rectangle { width: 60, height: 45};

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

因为定义一个方法，所以它应该位于impl Rectangle块中，方法名是can_hold，并且他会获取另一个不可变借用作为参数，通过观察调用位置的代码可以看出刹那火速是什么类型： 
rect1.can_hold（&rect2）传入&rect，它是一个Rectangle的rect2的不可变借用。

这是因为我们只需要读取rect2而不是写入，这意味者只需要一个不可变借用，而且希望main保持rect2的所有权， 这样就可以在调用这个方法后继续使用它。


    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance (&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared  + y_squared)
        }

        fn  can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

方法可以在self后增加多个参数，而且这些参数就像函数中的参数一样工作。

### 关联函数

impl块的另一个功能是： 允许在impl块中定义不以self作为参数的函数，这被称为关联函数，因为他们与结构体相关联。

因此他们是函数而不是方法，因为它们并不作用域一个结构体的实例。

**关联函数经常被用作返回一个结构体新实例的构造函数**

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle{
        fn square(size: u32) -> Rectangle{
            Rectangle { width: size, height: size}
        }
    }

**使用结构体和::语法来调用这个关联函数

### 多个impl块

每个结构体都允许拥有多个impl块

没有理由将方法分散在多个impl块中，不过这是有效的语法。

## 总结

结构体让我们可以在自己的范围内创建有意义的自定义类型，通过结构体，可以将相关联的数据片段联系起来并命名他们，这样可以使代码更加清晰。

方法允许为结构体实例指定行为，而关联函数将特定功能置于结构体的命名空间中并且无需一个实例。

机构体并不是创建自定义类型的唯一方法



      