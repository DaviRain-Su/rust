extern crate tour_of_rust;
fn main() {
    
    //test chapter1
    tour_of_rust::chapter1::variable_type();
    tour_of_rust::chapter1::change_variable_type();
    tour_of_rust::chapter1::basic_type();
    tour_of_rust::chapter1::as_type();
    tour_of_rust::chapter1::const_type();
    tour_of_rust::chapter1::array_type();
    let ret = tour_of_rust::chapter1::add(23, 12);
    println!("ret is {}", ret);
    let result = tour_of_rust::chapter1::swap(123, 321);
    println!("{} {}", result.0, result.1);

    let (a, b) = tour_of_rust::chapter1::swap(result.0,result.1);
    println!("{} {}", a, b);

    let a = tour_of_rust::chapter1::make_nothing();
    println!("The value of a is : {:?}", a);

    // test chapter2
    tour_of_rust::chapter2::control_flow();
    tour_of_rust::chapter2::loop_flow();
    tour_of_rust::chapter2::while_flow();
    tour_of_rust::chapter2::for_flow();
    tour_of_rust::chapter2::macro_flow();
    tour_of_rust::chapter2::return_value_from_loop();
    let ret = tour_of_rust::chapter2::return_block_expression();
    println!("return block expression is {}", ret);

    //test chapter3
    tour_of_rust::chapter3::function_way();


}

