#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn test1() {
    #[derive(Debug, Copy, Clone)]
    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            let x_squared = f64::powi(other.x - self.x, 2);
            let y_squared = f64::powi(other.y - self.y, 2);

            f64::sqrt(x_squared + y_squared)
        }
    }

    let p1 = Point { x: 0.0, y: 0.0 };
    println!("P1 is {:#?}", p1);
    let p2 = Point { x: 3.0, y: 4.0 };
    println!("p2 is {:#?}", p2);

    println!("this distance is {}", p1.distance(&p2));
}
fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user1
    };

    // let s = String::from("hello world");

    println!("user1 {:#?}", user1);
    println!("user2 {:#?}", user2);

    test1();
}
