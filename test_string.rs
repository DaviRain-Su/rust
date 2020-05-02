fn main() {
    //   let mut s = String::from("hello");

    //   s.push_str(", world!"); // push_str() append a literal to a string

    test1();

    //    println!("{}", s); // this will print "hello, world"
}

fn test1() {
    let s = String::from("Hello"); // s comes into scope

    takes_ownership(s); // s's value move into the function ..
                        // .. and so is no longer vaild here.

    //    println!("{}", s);
    let x = 5; //  x comes intp scope

    makes_copy(x); // x would move into the function.
                   // but i32 is copy, so it's okay to still
                   // use x afterward/
    println!("{}", x);
} // Here, x goes out of scope, then s. But since s's value was moved, nothing
  // special happens

fn takes_ownership(some_string: String) {
    // some_string comes into scope

    println!("{}", some_string);
} // Here, some_string goes out of scope and 'drop' is called. the backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope, Nothing special happens.
