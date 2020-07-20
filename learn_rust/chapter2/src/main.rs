extern crate chapter2;
fn main() {
    chapter2::basic_type::bool_type();
    chapter2::basic_type::number_type();
    chapter2::basic_type::char_type();
    chapter2::basic_type::array_type();
    chapter2::basic_type::range_type();
    chapter2::basic_type::slice_range();
    chapter2::binding::immutable_and_mutable();
    chapter2::binding::temp();
    chapter2::binding::ownership();
    chapter2::binding::reference();
    println!("Hello, world!");

    match t() {
        Ok(true) => println!("ok"),
        _ => println!("error"),
    }
}

fn t() -> Result<bool,()> {
    Ok(
        Ok(true)? && Ok(true)?
    )
}
