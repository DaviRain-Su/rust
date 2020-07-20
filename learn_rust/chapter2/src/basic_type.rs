/// # 基本数据类型
/// 
/// ## 布尔类型
/// 
/// Rust内置了布尔类型，类型名为bool.
/// bool类型只有两个值 -- true和false 
/// 
/// Basic Usage:
/// 
/// ```
/// fn bool_type(){
///     let x = true;
///     let y: bool = false;
///     let x = 5;
///     if x > 1 { println!("x is bigger than 1") };
///     assert_eq!(x as i32, 1);
///     assert_eq!(y as i32, 0);
/// }
/// ```
/// 对于x的绑定，rust可以自动推断其类型为bool，也可像y一样显式声明其指定的类型为bool
/// 任意一个比较操作都产生bool类型
/// 也可以通过as 操作符将bool类型转换成数字0和1,
/// 但是rust并不支持将数字转换为bool类型
pub fn bool_type(){
    let _x = true;
    let y: bool = false;
    let x = 5;
    if x > 1 {
        println!("x is bigger than 1");
    }
    // assert_eq!(x as i32, 1); // x is 5, this will apperse error
    assert_eq!(y as i32, 0);
}

/// ## 基本数字类型
/// 
/// Rust提供的基本数字类型大致分成三大类：固定大小类型、动态大小类型和浮点数
/// 分别介绍如下：
/// 
/// - 固定大小的类型包括无符号整数 unsigned integer和符号整数signed integer,
/// 其中无符号整数包括：
///     - u8, 占用1个字节， u8通常在rust中表示字节序列，在文件I/O或网络I/O中读取
///       数据流时需要使用u8.
///     - u16, 占用2个字节
///     - u32, 占用4个字节
///     - u64, 占用8个字节
///     - u128， 占用16个字节
/// 符号整数包括:
///     - i8, 占用1个字节
///     - i16, 占用2个字节
///     - i32, 占用4个字节
///     - i64, 占用8个字节
///     - i128, 占用16个字节
/// - 动态大小类型分为：
///     - usize, 占用4个或者8个字节， 具体取决于机器的字长。
///     - isize, 占用4个或者8个字节，具体取决于机器的字长。
/// - 浮点数类型分为：
///     - f32, 单精度32位浮点数，至少6位有效数字
///     - f64, 单精度64位浮点数，至少15位有效数字
/// 
/// Basic Usage:
/// 
/// ```
/// fn number_type(){
///     let num = 42u32;
///     let num: u32 = 42;
///     let num = 0x2A;
///     let num = 0o106;
///     let num = 0b1101_1011;
///     assert_eq!(b'*', 42u8);
///     assert_eq!(b'\'', 39u8);
///     let num = 3.1415926f64;
///     assert_eq!(-3.14, -3.14f64);
///     assert_eq!(2., 2.0f64);
///     assert_eq!(2e4, 20000f64);
///     println!("{:?}", std::f32::INFINITY);
///     println!("{:?}", std::f32::NEG_INFINITY);
///     println!("{:?}", std::f32::NAN);
///     println!("{:?}", std::f32::MIN);
///     println!("{:?}", std::f32::MAX);
/// }
/// ```
/// 创建的数字字面量后面可以直接使用类型后缀，42u32。
/// 如果不加后缀或者没有指定类型，Rust编译器会mor推断数字为i32类型
/// 
/// 可以用前缀0x, 0o, 0b分别表示十六进制，八进制，二进制
/// 
/// Rust也可以写字面量，比如b'*'
/// 
/// 浮点数也可以为字面量加类型后缀，如果不加类型后缀或没有指定类型，rust会默认
/// 推断浮点数为f64类型，标准库std::f32和std::f64都提供量IEEE所需的特殊
/// 常量值，比如INFINITY, NEG_INFINITY, NAN, MIN, MAX.
/// 
pub fn number_type(){
    let _num = 42u32;
    let _num: u32 = 42;
    let _num = 0x2A;
    let _num = 0o106;
    let _num = 0b1101_1011;
    assert_eq!(b'*', 42u8);
    assert_eq!(b'\'', 39u8);
    let _num = 3.1415926f64;
    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);
    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX)
}

/// ## 字符类型
/// 
/// 在Rust中，使用单引号来定义字符类型。字符类型代表的是一个Unicode标量值，每个字符占4个字节
/// 
/// Basic Usage:
/// 
/// ```
/// fn char_type(){
///     let x = 'r';
///     println!("{}", '\'');
///     println!("{}", '\\');
///     println!("{}", '\n');
///     println!("{}", '\r');
///     println!("{}", '\t');
///     assert_eq!('\x2A', '*');
///     assert_eq!('\x25', '%');
///     assert_eq!('%' as i8, 37);
/// }
/// ```
/// 字符可以使用ASICII码和Unicode码来定义， '2A'为ASCII码表中表示符号'*'的十六进制
/// 数，格式为'\xHH',
/// 可以使用as操作符将字符转为数字类型
pub fn char_type(){
    let _x = 'r';
    println!("{}", '\'');
    println!("{}", '\\');
    println!("{}", '\n');
    println!("{}", '\r');
    println!("{}", '\t');
    assert_eq!('\x2A', '*');
    assert_eq!('\x25', '%');
    assert_eq!('%' as i8, 37);
}

/// ## 数组类型
/// 
/// 数组是Rust内建的原始集合类型，数组的特点为：
/// - 数组大小固定
/// - 元素均为同类型
/// - 默认不可变
/// 
/// 数组的类型签名[T : N]. T是一个泛型标记，代表数组中元素的某个具体类型，
/// N代表数组的长度， 是一个编译时常量，必须在编译时确定其值。
/// 
/// Basic Usage: 
/// ```
/// fn array_type(){
///     let arr: [i32: 3] = [1, 2, 3];
///     let mut mut_arr = [1, 2, 3];
///     assert_eq!(1, mut_arr[0]);
///     mut_arr[0] = 3;
///     assert_eq!(3, mut_arr[0]);
///     let init_arr = [0;10];
///     assert_eq!(0, init_arr[5]);
///     assert_eq!(0, init_arr.len());
///     // println!("{:?}", arr[5]); // Error, index out of bounds
/// }
/// ```
/// 定义的类型[i32; 5]的数组，该数组是固定长度的，不允许对其添加或删除元素，即使通过let mut 关键字定义可变绑定
/// mut arr, 也只能修改已存在于索引位置上的元素。
/// 
/// 通过[0;10]语法会创建初始值为0且指定长度为10的数组，对于越界访问的情况，Rust会报错，有效阻止来内存不安全的操作。
/// 
/// 对于原始固定长度数组，只有实现Copy trait 的类型才能作为其元素。未来Rust还将支持
/// VLA（variable-length array）数组，即可变长度数组。对于可变长度数组，将会基于可以在栈上动态内存的函数来实现。
pub fn array_type(){
    let _arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [1, 2, 3];
    assert_eq!(1, mut_arr[0]);
    mut_arr[0] = 3;
    assert_eq!(3, mut_arr[0]);
    let init_arr = [0; 10];
    assert_eq!(0, init_arr[5]);
    assert_eq!(10, init_arr.len());
    //println!("{:?}"m arr[5]); // Error; index out of bounds

} 

/// ## 范围类型
/// 
/// Rust内置来范围类型，包括左闭右开和全闭两种区间。
/// 
/// Basic Usage: 
/// ```
/// fn range_type(){
///     assert_eq!((1..5), std::ops::Range(start: 1, end: 5));
///     assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
///     assert_eq!(3+4+5, (3..6).sum());
///     assert_eq!(3+4+5+6, (3..=6).sum());
///     for i in (1..5) {
///         println!("{}", i); //1, 2, 3, 4    
///     }
///     for i in (1..=5) {
///         println!("{}", i); //1, 2, 3, 4, 5        
///     }
/// }
/// ```
/// (1..5)表示左闭右开区间，(1..=5)表示全闭区间， 分别是std::ops::Range 和 std::ops::RangeInclusive的实例。
/// 每一个范围都是一个迭代器，可以直接使用for循环打印。
/// 
pub fn range_type(){
    assert_eq!((1..5), std::ops::Range{ start: 1, end: 5});
    assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
    assert_eq!(3+4+5, (3..6).sum());
    assert_eq!(3+4+5+6,(3..=6).sum());

    for i in (1..5) {
        println!("{}", i);
    }

    for i in (1..=5) {
        println!("{}", i);
    }
}

/// ##  切片类型
/// 切片类型是对数组（包括固定大小数组和动态数组）的引用片段，有利于
/// 安全有效地访问数组的一部分，而不需要拷贝操作。因为理论上，切片引用的是
/// 已经存在的变量。在底层，切片代表一个指向数组起始位置的指针和数组长度。
/// 用'['T']'类型表示连续序列，那么切片类型就是&'['T']', &mut '['T']'
/// 
/// Basic Usage
/// 
/// ```
/// fn slice_range() {
///     let arr : [i32; 5] = [1, 2, 3, 4, 5];
///     assert_eq!(&arr, &[1, 2, 3, 4, 5]);
///     assert_eq!(&arr[1..], [2, 3, 4, 5]);
///     assert_eq!(&arr.len(), &5);
///     assert_eq!(&arr.is_empty(), &false);
///     let arr  = &mut [1, 2, 3];
///     arr[1] = 7;
///     assert_eq!(arr, [1, 7, 3]);
///     let vec = vec![1, 2, 3];
///     assert_eq!(&vec[..], [1, 2, 3]);
/// }
/// ```
/// 通过引用操作符&对数组进行引用，就产生了一个切片&arr
/// 也可以结合范围对数组进行切割, &arr[1..], 表示获取arr数组中在索引位置1之后的所有元素。
/// 切片提供了两个const fn 方法，len 和 is_empty(),分别用来得到切片的长度和判断切片是否为空。
/// 通过&mut可以定义可变切片，这样可以直接通过索引修改相应位置的值。
/// 对于使用vec!宏定义的动态数组，也可以通过引用操作符来得到一个切片
/// 
pub fn slice_range() {
    let arr : [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2, 3, 4, 5]);
    assert_eq!(&arr[1..], &[2, 3, 4, 5]);
    assert_eq!(&arr.len(), &5);
    assert_eq!(&arr.is_empty(), &false);
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);
    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1, 2, 3]);
}