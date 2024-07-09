use rand::random;
use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::iter::from_fn;
use std::path::Path;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
    let n = 10;
    let result = trangle(n);
    println!("The result is: {}", result);

    // use next to iterate over a vector
    let v = [4, 20, 12, 8, 6];
    let mut iterator = v.iter();
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), Some(&20));
    assert_eq!(iterator.next(), Some(&12));
    assert_eq!(iterator.next(), Some(&8));
    assert_eq!(iterator.next(), Some(&6));
    assert_eq!(iterator.next(), None);

    // use next to iterate over a path
    let path = Path::new("/Users/davirian/dev/rust/programming-rust/ch15-iterator");
    //println!("Path:{:?}", path);
    let mut iterator = path.iter();
    assert_eq!(iterator.next(), Some(OsStr::new("/")));
    assert_eq!(iterator.next(), Some(OsStr::new("Users")));
    assert_eq!(iterator.next(), Some(OsStr::new("davirian")));
    assert_eq!(iterator.next(), Some(OsStr::new("dev")));
    assert_eq!(iterator.next(), Some(OsStr::new("rust")));
    assert_eq!(iterator.next(), Some(OsStr::new("programming-rust")));
    assert_eq!(iterator.next(), Some(OsStr::new("ch15-iterator")));
    assert_eq!(iterator.next(), None);

    let mut favorites = BTreeSet::new();
    favorites.insert("Lucy in the sky with diamonds".to_string());
    favorites.insert("Liebestraume No. 3".to_string());
    favorites.insert("Bach, Brandenburg Concerto No. 3 G Major".to_string());
    let mut it = favorites.iter();
    assert_eq!(
        it.next(),
        Some("Bach, Brandenburg Concerto No. 3 G Major".to_string()).as_ref()
    );
    assert_eq!(it.next(), Some("Liebestraume No. 3".to_string()).as_ref());
    assert_eq!(
        it.next(),
        Some("Lucy in the sky with diamonds".to_string()).as_ref()
    );
    assert_eq!(it.next(), None);

    println!("{:?}", favorites);

    let v = vec![4, 20, 12, 8, 6];
    dump(v);

    let length: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect();
    dump(length);

    assert_eq!(
        fibonacci().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );
    println!("{:?}", fibonacci().take(8).collect::<Vec<_>>());

    // drain
    let mut outer = "Earth".to_string();
    let inner = String::from_iter(outer.drain(1..4));

    assert_eq!(outer, "Eh");
    assert_eq!(inner, "art");
    println!("outer: {}, inner: {}", outer, inner);

    // map
    let text = "    ponies  \n  giraffes\niguanas   \nsquid".to_string();
    let v: Vec<&str> = text.lines().map(str::trim).collect();
    assert_eq!(v, vec!["ponies", "giraffes", "iguanas", "squid"]);
    println!("{:?}", v);
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, vec!["ponies", "giraffes", "squid"]);
    println!("{:?}", v);

    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    let numbers: Vec<f64> = text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
        .collect();

    for num in numbers {
        println!("{:4.2}", num.sqrt());
    }

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Brasilia"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("Netherlands", vec!["Amsterdam", "Utrecht"]);

    let countries = ["Japan", "Brazil", "Kenya"];

    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    let mut parks = BTreeMap::new();
    parks.insert("Portland", vec!["Mt. Tabor", "Washington"]);
    parks.insert("Nashville", vec!["Centennial", "Shelby"]);
    parks.insert("Sao Paulo", vec!["Ibirapuera", "Pinheiros"]);

    let all_parks: Vec<_> = parks.values().flatten().cloned().collect();
    println!("{:?}", all_parks);

    let result = vec![None, Some("day"), None, Some("one"), None, None]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    println!("{:?}", result);

    let template_string = "hello, world";
    let result = template_string
        .chars()
        //.map(|c| c.to_uppercase())
        //.flatten()
        .flat_map(|c| c.to_uppercase())
        .collect::<String>();
    println!("{:?}", result);
}

pub fn trangle(n: i32) -> i32 {
    // use fold to calculate the sum of 1..=n
    (1..=n).sum::<i32>()
}

pub fn dump<T, U>(iter: T)
where
    T: IntoIterator<Item = U>,
    U: std::fmt::Debug,
{
    for i in iter {
        println!("{:?}", i);
    }
}

pub fn fibonacci() -> impl Iterator<Item = u64> {
    let mut state = (0, 1);
    from_fn(move || {
        let next = state.0 + state.1;
        state = (state.1, next);
        Some(state.0)
    })
}
