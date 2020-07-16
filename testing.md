# 测试

## 如何编写测试

测试用来验证非测试的代码是否按照期望的方式运行的rust函数。

测试函数体通常需要执行如下三种操作：

1. 设置任何所需的数据或状态

2. 运行需要测试的代码

3.断言其结果是我们期望的。

rust提供的专门用来编写测试的功能： test属性、一些宏和should_panic 属性

## 测试函数剥析

rust中的测试就是一个带有test属性注解的函数，属性是关于rust代码片段的元数据，

为来将一个函数变成测试函数，需要在fn行之前加上 #[test].

当使用cargo test 命令运行测试函数时，rust会构建一个测试执行二进制文件用来运行标记来test属性的函数并报告每一个测试是通过还是失败。

        #[cfg(test)]
        mod test {
            #[test]
            fn it_works() {
                assert!(2 + 2, 4);
            }
        }

注意fn行之前的#[test]这个属性表明这是一个测试函数，这样测试执行者就知道将其作为测试处理，因为也可以在tests模块中拥有非测试的函数来帮助我们建立通用场景进行常见操作，所以需要使用#[test]属性表明那些函数是测试。

当测试函数中出现panic时测试就失败来，每一个测试都在一个新线程中运行，当主线程发现测试线程异常来，就会将对应测试标记为失败。

## 使用assert!宏来检查结果

assert！宏由标准库提供，在希望确保测试中一些条件为true时非常有用，需要向assert！宏提供一个计算为布尔值的参数，如果是true，assert！什么也不做同时测试会通过。如果值为false，assert！调用宏，这回导致测试失败。assert！宏帮助我们检查代码中是否以期望的方式运行。

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn larger_can_hold_smaller() {
            let larger = Rectangle { length: 8, width: 7};
            let smaller = Rectangle { length: 5, width: 1};

            assert!(larger.can_hold(&smaller));
        }
    }

use super:: * ; test是一个普通模块，这是一个内部模块，需要将外部模块中被测试的代码引入到内部模块的作用域中。这里选择使用全局导入使得外部模块定义的所有内容在tests模块中都是可用的。

## 使用assert_eq!和assert_ne!宏来测试相等

测试功能的一个常用方法是将需要测试代码的值与期望值做比较，并检查是否相等。可以通过向assert！宏传递一个使用 == 运算符的表达式来做到，不过这个操作太常见了，以至于标准库提供了一对宏来更方便的处理这些操作， assert_eq!和 assert_ne! 这两个宏分别比较两个值是相等还是不相等。当断言失败他们也会打印出这两个值具体是什么。
而assert！只会打印出它从==表达式中得到来false值，而不是导致false的两个值

    pub fn add_two(a: i32) -> i32 {
        a + 2
    }

    #[cfg(test)]
    mod tests{
        use super::*;
        #[test]
        fn it_add_two(){
            assert_eq!(4, add_two(2));
        }
    }

assert_ne!宏在传递它的两个值不相等时通过而在相等时失败，这个宏在代码按照我们期望运行时不确定值会是什么，不过知道他们绝对不会是什么的时候最有用处。

assert_eq! 和assert_ne! 宏在底层分别使用来 == 和 != 。当断言失败时，这些宏会使用调试格式打印出其参数，这意味则会被比较的值必须实现了PartialEq和 Debug trait。所有的基本类型和大部分标准库类型都实现了这些trait，对于自定义的结构体和枚举，需要实现partialEeq，才能断言他们的值是否相等。需要实现Debug才能在断言失败时打印出他们的值。因为这两trait都是派生trait。

## 自定义错误信息

可以向assert! assert_eq! 和 assert_ne! 宏传递一个可选的参数来增加用于打印的自定义错误信息。任何在assert!必需的一个参数和assert_eq! 和 assert_ne! 必需的两个参数之后指定的参数都会传递format!宏，所以可以传递一个包含{} 占位符的个死字符串和放入占位符的值，自定义信息有助于记录断言的意义，这样到测试失败时，就能更好的理解代码出了什么问题。

    pub fn greeting(name: &str) -> String {
        format!("Hello {}!", name)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn greeting_contains_name(){
            let reuslt = greeting("Carol");
            assert!(result.contains("Carol"));
        }
    }

    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
        "Greeting did not contains name, value was '{}'", result);
    }


## 使用should_panic 检查panic


