# 使用模块组织和复用代码

rust有一个模块系统可以有组织地复用代码。

可以将函数和其他类似结构体和枚举的代码提取到不同模块中。

模块是一个包含函数或类型定义的命名空间，可以选择这些定义共有或私有，在其模块外的可见性。

- 使用mod关键字声明新模块。此模块中的代码要么直接位于声明之后的大括号中，要么位于另一个文件。

- 函数、类型、常量和模块默认都是私有的。可以使用pub关键字将其编程共有并在其命名空间之外可见。

- use关键字将模块或模块中的定义引入到作用域中以便引用他们。

## mod和文件系统

学习根据编写代码的意图来以不同方法组织库项目代码以适应多种情况。

首先定义一个叫做network的模块，它包含一个叫做connect的函数定义。
rust中所有模块的定义都以关键字mod开始。

    mod network {
        fn connect() {

        }
    }
mod 关键字后面是模块的名字，network，位于大括号中的代码块。

代码块中的一切都位于network命名空间中。

在这个例子中，只有一个函数，connect。如果想要在network模块外面的代码中调用这个函数，需要指定模块名并使用命名空间语法::, network::connect().

也可以在lib.rs文件中同时存在多个模块。

    mod network {
        fn conect() {

        }
    }

    mod client {
        fn connect() {

        }
    }

也可以将此模块放入其他模块中，有助于随着模块的增长，将相关的功能组织在一起并保持各自独立。

如何选择组织代码依赖于如何考虑代码不同部分之间的关系。

    mod network {
        fn conect() {

        }

        mod client {
            fn connect() {

            }
        }
    }

有了  network::connect和 network::connect函数，这两个connect函数也不冲突

这两个conect函数也不冲突，因为他们在不同的命名空间中。

### 将模块移动到其他文件

可以利用rust的模块系统连同多个文件一起分解rust项目，这样就不糊是所有内容落在src/lib.rs 或src/main.rs

    mod client {
        fn connect() {

        }
    }

    mod network {
        fn connect() {

        }

        mod server {
            fn connect() {

            }
        }
    }

如果这些模块中有很多函数，而这些函数又很长，将难以在文件中寻找我们需要的代码。因为这些函数被嵌套进一个或多个mod块中。同时函数中的代码也会开始变长。

    mod client;

    mod network {
        fn connect() {

        }

        mod server {
            fn connect() {

            }
        }
    }

仍然声明了client模块，不过将代码替换为了分号，这告诉rust在client模块的作用域中寻找另一个代码的位置。

mod client；意味着。

    mod client {
        // contents of client.rs
    }

需要创建对应模块的外部文件，在src/目录创建一个client.rs文件，接着打开它并输入被去掉client模块中的connect函数。

    fn connect() {

    }

**这个文件中并不需要一个mod声明；因为已经在src/lib.rs中已经使用了mod声明了cliet模块。这个文件仅仅提供client模块的内容。如果在这里加上一个mod client，那么就等于给client模块增加了一个叫做client的子模块。**


rust默认只知道src/lib.rs中的内容。如果想要对项目加入更多文件。需要在src/lib.rs中告诉rust去寻找其他文件；

### 模块文件系统的规则

- 如果一个叫做foo的模块没有子模块，应该将foo的声明放入叫做foo.rs的文件中
- 如果一个叫做foo的模块有自模块，应该将foo的声明放入叫做foo/mod.rs的文件中。

## 使用pub控制可见性

    extern crate communicator;

    fn main() {
        communicator::client::connect();
    }

使用extern crate 指令将 communicator 库引入到作用域。

大部分功能位于库crate中，而二进制crate使用这个库crate。

从一个外部crate的视角观察communicator库的内部，我们创建的所有模块都位于一个与crate同名的模块内部，communicator。这个顶层的模块被称为crate的根模块。

另外注意到即便在项目的子模块中使用外部crate，extern crate也应该位于根模块。接着，在自模块中，我们可以像顶层模块那样引用外部crate中的项了。

如果不在自己的项目中使用一个私有函数，因为程序自身是唯一允许使用这个函数的代码，rust会警告说函数未被使用。

当某项被标记为公有，rust不再要求它在程序自身被使用并停止警告函数未被使用。

### 标记函数为共有

为了告诉rust将函数标记为公有，在声明的开头增加pub关键字。

未被使用的代码并不总是意味着他们需要被设为共有；如果你不希望这些函数成为共有API，
未被使用的代码警告可能是在提醒你这些代码不再需要并可以安全的删除他们。

这也可能是警告你出bug了，如果你不小心删除了库中所有这个函数的调用。

### 私有性规则

1 如果一个项是公有的，他能被任何父模块访问
2 如果一个项是私有的，他能被其直接父模块及其任何子模块访问

    mod outermost {
        pub fn middle_function() {}

        fn middle_secret_function() {}

        mod inside {
            pub fn inner_function() {}
            fn secret_function() {}
        }
    }

    fn tey_me() {
        outermost::middle_function();
        outermost::middle_secret_function();
        outermost::inside::inner_function();
        outermost::inside::secret_function();
    }


try_me 函数位于项目的根模块，叫做outermost的模块是私有的，
第二条私有行规则说明try_me 函数允许访问outermost模块，因为outermost位于当前（根）模块，try_me也是。

outermost::middle_function的函数调用是正确的，因为middle_function是共有的，而try_me通过父模块outermost访问middle_function.根据上一段的规则我们可以确定这个模块是可访问的。

outermost::middle_secret_function的调用会造成一个编译错误。middle_secret_function 是私有的，所以第二条（私有性）规则生效了，模块既不是middle_secret_function的当前模块，也不是middle_secret_function当前模块的子模块。

叫做inside 的模块是私有的且没有子模块，所以它只能被当前模块outermost访问。这意味着try_me函数不允许调用outermost::inside::inner_function或outermost::inside::secret_function中的任何一个。

## 引用不同模块中的名字

    pub mod a {
        pub mod series {
            pub mod of {
                pub fn nested_modules() {

                }
            }
        }
    }

    fn  main() {
        a::series::of::nested_modules();
    }
    
### 使用use关键字将名字导入作用域


rust的use关键字能将你想要调用的函数所在的模块引入到当前作用域中，通过这种方式可以缩短冗长的函数调用。

     pub mod a {
        pub mod series {
            pub mod of {
                pub fn nested_modules() {

                }
            }
        }
    }

    use a::series::of;

    fn  main() {
       of::nested_modules();
    }

use a::series::of;这一行的意思是每当想要引用of模块时，不必使用完整的a::series::of路径，可以直接使用of，可以直接使用of。

use关键字只将指定的模块引入作用域；它并不会将其子模块也引入。

也可以将函数本身引入到作用域中，

    pub mod a {
        pub mod series {
            pub mod of {
                pub fn nested_modules() {

                }
            }
        }
    }

    use a::series::of::nested_modules;

    fn  main() {
       nested_modules();
    }

因为枚举也像模块一样组成了某种命名空间，也可以使用use来导入枚举成员。

对于任何类型的use语句，如果从一个命名空间导入多个项，可以在最后使用打括号和逗号来列举他们。

    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    use TrafficLight::{ Red, Yellow};

    fn main() {
        let red = Red;
        let yellow = Yellow;
        let green = TrafficLoght::Green;
    }
### 使用glob将所有名称引入作用域

为来一次将某个命名空间下的所有名称都引入作用域，可以使用 * 语法。这称为glob运算符。

    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    use TrafficLight::* ;

    fn main() {
        let red = Red;
        let yellow = Yellow;
        let green = Green;
    }

* 会将TrafficLoght 命名空间中所有可见的项都引入作用域。应当保守使用glob。
可能会引入多于预期的内容而导致命名冲突。

###    使用super访问父模块


    #[cfg(test)]
    mod tests{
        use super::client;

        #[test]
        fn it_works() {
            client::connect();
        }
    }
use 语句，默认是相对于crate根模块。test模块需要client模块位于其作用域中。

    ::client::connect();

    super::client::connect();

super来获取当前模块的同级模块是一个好的捷径。
如果在代码中很多地方指定来从根开始的路径，那么当通过移动子树或到其他位置来重新排列模块时，最终就需要更新很多地方的路径。

super::    的功能改变来提供给use的路径，使其不再相对于根模块而是相对于父模块。

