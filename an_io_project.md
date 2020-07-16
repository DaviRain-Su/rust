# 一个IO项目： 构建一个命令行程序

## 接受命令行参数

## 读取参数值

为了确保minigrep能够获取传递给它的命令行参数的值，我们需要一个rust标准库提供的函数， std::env::args,这个函数返回一个传递给程序的命令行参数的迭代器。

只需理解迭代器的两个细节： 迭代器生成一系列的值，可以在迭代器上调用collect方法将其转换为一个集合，比如包含所有迭代器产生元素的vector。


    use std::env;

    fn  main() {
        let agrs: Vec<String> = env::args().collect();
        println!("{:?}", args);
    }

首先使用use语句将std::env模块引入作用域以便可以使用它的args函数。注意std::env::args函数被嵌套进了两层模块中，当所需函数嵌套了多于一层模块时，通常将父模块引入作用域，而不是其自身。这便于我们利用std::env中的其他函数， 这比增加 use std::env::args;后仅仅使用args调用函数要更明确一些。因为args容易被错认为一个定义于当前模块的函数。

在卖弄韩式的第一行，调用了env::args,并理解使用collect来创建一个包含迭代器所有值的vector。collect可以被用来创建很多类型的集合，所以这里显式注明args的类型来指定我们需要一个字符串vector，虽然在rust中我们很少会需要注明类型，collect就是一个经常需要注明类型的函数，因为rust不能推断出你想要什么类型的集合。

## 将参数值保存进变量


    use std::env;

    fn  main() {
        let agrs: Vec<String> = env::args().collect();

        let query = &args[1];
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);
        //println!("{:?}", args);
    }

## 读取文件

读取由命令行参数指定的文件。

    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    fn  main() {
        let agrs: Vec<String> = env::args().collect();

        let query = &args[1];

        // ----- snip -------
        
        let filename = &args[2];

        println!("Searching for {}", query);
        println!("In file {}", filename);

        let mut f = File::open(filename).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
        
        println!("with text : \n {}", contents);

    }

增加来更多的use语句来引入标准库中的相关部分，需要std::fs::File 来处理文件，而std::io::prelude::*; 则包含来许多对于IO包活文件IO有帮助的trait。类似于rust有一个通用的prelude来自动引入特定内容，std::io也有其自己的prelude来引入处理IO时所需的通用内容。不同于默认的prelude，必须显示use位于std::io中的prelude。

第一通过传递变量filename 的值调用file::open函数来获取文件的可变句柄。创建来叫做contents的变量并将其设置为一个可变的，空的string。它将会存放之后读取的文件内容。第三对文件句柄调用read_to_string并传递contents的可变引用作为参数。



main函数有这多个只能，通常函数只负责一个功能的话会更简洁并易于维护。

## 重构改进模块性和错误处理

