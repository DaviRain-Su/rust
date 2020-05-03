# 通用集合类型

集合指向的数据是存储在堆上的，这意味这数据的数量不必在编译时就已知并且可以随着程序的运行曾长或缩小。

vector允许我们一个挨着一个地存储一系列数量可变的值
字符串是一个字符的集合，
哈希map允许我们将值与一个特定的键相关联。这是一个叫做map的更为通用的数据结构的特定实现

## vector 用来存储一系列的值

vector允许我们在一个单独的数据结构中存储多于一个的值，它在内存中彼此响铃地排列所有的值。

vector只能存储相同类型的值。

### 新建vector

创建一个新的空的vector，可以调用Vec::new()

    let v: Vec<i32>  = Vec::new();

一旦插入值rust就可以推断出想要存放的类型，

    let v = vec![1, 2, 3];

### 更新vector

    let mut v = Vec::new();

    v.push(5);
    v.push(7);

### 丢弃vector时也会丢弃所有元素

    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    }// v goes out of scope and is freed here

当vector被丢弃时，所有其内容也会被丢弃，这意味着它包含的整数将会被清理。

### 读取vector的元素

索引语法或者get方法

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

使用& 和 [] 返回一个引用

或者使用get方法以索引为参数来返回一个Option<&T>

rust  有两个引用元素的方法的原因是程序可以选择如何处理当索引值在vector中没有对应值的情况。

    let v = vec![1, 2, 3, 4, 5];
    
    let does_no_exit = &v[100];
    let does_no_exit = v.get(100);

当运行，第一个[] 方法，当引用一个不存在的元素时Rust会造成panic!.
这个方法更适合当程序认为尝试访问超过vector结尾的元素是一个严重错误的情况，这时应该使程序崩溃。

当get方法被传递来一个数组外的索引时，它不会panic而是返回None。
当偶尔出现超过vector范围的访问属于正常情况时可以考虑使用它，接着在代码可以有处理
Some(&element)或None的逻辑。

### 无效引用

一旦程序获取来一个有效的引用，借用检查器会执行所有权和借用规则来确保vector内容的这个引用和任何其他引用保持有效。


不能在相同作用域中同时存在可变和不可变引用的规则。

    let mut v = vec![1, 2, 3, 4, 5];
    
    let first = &v[0];
        // immutable borrow occurs here
    v.push(6);
        // mutable borrow occurs here

在vector的结尾增加新的元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时， 第一个元素的引用就指向来被释放的内存，借用规则阻止来程序陷入这种情况。

### 遍历vector中的元素


    let v = vec![100, 32, 57];
    for i in &v { // 不可变借用
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {// 可变借用
        *i += 50; 
    }

为来修改可变引用所指向的值，在使用+= 运算符之前必须使用解引用运算符 * 获取 i中的值

### 使用枚举来存储多种类型

枚举的成员都被定义为相同的枚举类型，所以当需要在vector中存储不同类型的值时，可以定义并使用一个枚举。


    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]

rust在编译时就必须准确知道vector中的类型原因在于它需要知道存储每个元素到底需要多少内存。

第二个好处可以准确的知道这个vector汇总允许什么类型。如果rust允许vector存放任意类型，那么当对vector元素执行操作时一个或多个类型的值就有可能会造成错误。

使用枚举外加match意味着rust能在编译时就能保证总是会处理所有可能的情况。

如果在编写程序时不能确切无遗地知道运行时会存储vector的所有类型，枚举就不行了。可以使用trait对象。

## 字符串存储了UTF-8编码的文本

rust的核心语言中只有一种字符串类型str， 字符串slice，通常以被借用的形式出现，&str。

rust中的字符串，他们通常指的是String和字符串slice &str类型，而不仅仅是其中之一。

### 新建字符串

    let mut s = String::new();

通常字符串会有初始化数据，因为我们希望一开始就有这个字符串，可以使用to_string 方法，他能用于任何实现Display trait的类型，字符串字面值也可以。

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly
    let s = "initial contents".to_string();


    let s = String::from("initial contents");

字符串是utf-8 编码的，所以可以包含任何可以正确编码的数据

### 更新字符串

#### 使用push附加字符串

    let mut s = String::from("foo");
    s.push_str("bar");

push_str 方法获取字符串slice，因为我们并不需要获取参数的所有权

    let mut s = String::from("lo");
    s.push('l');

####    使用 + 运算符或format!宏连接字符串


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note that s1 has been movec here and can not longer be used

s1在相加后不再有效的原因，和s2的引用的原因与使用+运算符时调用的方法签名有关，

    fn add(self, s: &str) -> String 

它看起来好像生成了很多拷贝不过实际上并没有；这个实现比拷贝要更高效。

     let s1 = String::from("tic");
     let s2 = String::from("tac");
     let s3 = String::from("toe");

     let s = S1 + "-" + &s2 + "-" + &s3;
    
     let s1 = String::from("tic");
     let s2 = String::from("tac");
     let s3 = String::from("toe");

     let s = format!("{}-{}-{}", s1, s2, s3);

format! 与 println! 的工作远离相同，不过不同于将输出打印到屏幕上，它返回一个带有结果内容的String。 并且不会获取任何参数的所有权。




### 索引字符串

    let s1 = String::from("Hello");
    let h = s1[0];


字节 标量 和字形簇

最后一个rust不允许使用索引获取string字符的原因是索引操作预期总是需要常数时间，

但是对于string不可能保证这样的性能，因为rust不得不检查从字符串的开头到索引位置的内容来确定这里有多少有效的字符。

### 字符串slice

为来更明确索引并表明你需要一个字符串slice，相比使用 [] 和单个值的索引，可以使用 
[] 和一个range来创建含特定字节的字符串slice

但是，应该小心使用这个操作，因为他可能会使你的程序崩溃。

### 遍历字符串的方法

如果我们需要操作单独的uincode标量值，最好的选择是使用chars方法。

bytes方法返回每一个原始字节

# 哈希map存储键值对

hashMap<K, V> 类型存储来一个键类型K对应一个值类型v的映射。它通过一个哈希函数来实现映射，决定 如何将键和值放入内存中。

## 新建一个哈希map

    let std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

必须首先use标准库中集合部分的hashMap。

同样类似于vector，哈希map是同质的；所有的键必须是相同类型，值也必须是相同类型。

另一个构建map的方法是使用一个元组的vector的collect方法，其中每个元组包含一个键值对。collect方法可以将数据收集进一系列的集合类型，包括Hashmap。

例如队伍的名字和初始分数分别在两个vector中，可以使用zip方法创建一个元组的vector，其中blue与10是一对，接着就可以使用collect方法将这个元组vecgor转换成一个hashmap。

    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

这里的HashMap<_, _>,类型注解是必要的，因为可能collect很多不同的数据结构，而除非显式指定否则rust无从得知你需要的类型。但是对于键和值的类型参数来说，可以使用下划线占位，而rust能够根据vector中的数据的类型推断出HashMap所包含的类型。

## 哈希map和所有权

    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map  = HashMap::new();
    map.insert(field_name, field_value);

    //field_name and field_value are invalid at this point, try using and see what compiler error your get!

当insert调用field_name和field_value移动到哈希map中后，将不能使用这两个绑定。

如果将值的引用插入哈希map，这些值本身将不会被移动进哈希map，但是这些引用指向的值必须至少在哈希map有效时也是有效的。

## 访问哈希map中的值

可以通过get方法并提供对应的键来从哈希map中获取值

    use std::collect::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

get返回Option<V>, 所以结果被装进some；如果某个键在哈希map中没有对应的值，get会返回None。

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

## 跟新哈希map

当想要改变哈希map中的数据时，必须决定如何处理一个键已经有值来的情况。可以选择完全无视旧值并用新值代替旧值，可以选择保留旧值而忽略新值，并只在键没有对应值时新增加新值。或者可以结合新旧两值。

### 覆盖一个值

如果插入一个键值对，接着用相同的键值对插入一个不同的值，与这个键值对相关联的旧值将被替换。

    use std::collections::HashMap;

    let mur scores = HashMap::new();

    socres.insert(Stirng::from("Blue"),10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);


### 只在键没有对应值时插入

检查某个特定的键是否有值，如果没有就插入一个值，为此hashmap又一个特有的API， 叫做entry， 它获取我们想要检查的键作为参数，entry函数返回值是一个枚举， Entry，它代表来可能存在也可能不存在的值。

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    socres.insert(String::from("Blue"), 10);

    socres.entry(String::from("Yellow")),or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    println!("{:?}", scores);

Entry的or_insert 方法在键值对对应的值存在时就返回这个值的Entry，如果不存在则将参数作为新值插入并返回修改过的Entry。


### 根据旧值更新一个值

找到一个键对应的值并根据旧值更新它。

例如，计数一些文本中每一个单词分别出现来多少次，

    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.spilt_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


or_insert方法事实上会返回这个键的值的一个可变引用。将这个可变引用存储在count变量中，所以为了赋值必须首先使用星号 * 解引用。这个可变引用在for循环的结尾离开作用域，这样所有这些改变都是安全的并符合借用规则。
