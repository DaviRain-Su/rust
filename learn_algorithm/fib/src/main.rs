
fn main() {
    let n  = 6; 
    let ret = fib(n);
    println!("fib({}) is {}", n, ret.0);
    println!("Hello, world!");
}

fn fib(n: u128) -> (u128, u128) {
    if n > 0 {
        let pf = fib(n / 2);
        let t0 = pf.0;
        let t1 = pf.1;
        if n % 2 == 1 {
            (t0 * t0 + t1 * t1, (2 * t0 + t1) * t1)
        }else{
            ((2 * t1 - t0) * t0 , t0 * t0 + t1 * t1)
        }
    }else{
        (0, 1)
    }
}
