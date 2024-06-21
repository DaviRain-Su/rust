use std::{sync::OnceState, thread};

#[derive(Debug)]
pub struct City {
    name: String,
    population: i64,
    country: String,
}

impl City {
    pub fn new(name: &str, population: i64, country: &str) -> City {
        City {
            name: name.to_string(),
            population,
            country: country.to_string(),
        }
    }

    pub fn get_statistic(&self, state: Statistic) -> i64 {
        match state {
            Statistic::Min => self.population,
            Statistic::Max => self.population,
            Statistic::Median => self.population,
            Statistic::Mean => self.population,
        }
    }
}

fn sort_cities(cities: &mut Vec<City>) {
    // Use function
    //cities.sort_by(city_population_descending);
    // 也可以使用闭包
    cities.sort_by(|city1, city2| city2.population.cmp(&city1.population))
}

/// 按照人口数量对城市进行排序的辅助函数
pub fn city_population_descending(city1: &City, city2: &City) -> std::cmp::Ordering {
    city2.population.cmp(&city1.population)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Statistic {
    Min,
    Max,
    Median,
    Mean,
}

pub fn sort_by_statistic(cities: &mut Vec<City>, stat: Statistic) {
    cities.sort_by_key(|city| -city.get_statistic(stat))
}

fn start_sorting_thread(mut cities: Vec<City>, stat: Statistic) -> thread::JoinHandle<Vec<City>> {
    let key_fn = move |city: &City| -> i64 { -city.get_statistic(stat) };

    thread::spawn(move || {
        cities.sort_by_key(key_fn);
        cities
    })
}

fn main() {
    //println!("Hello, world!");
    let mut cityes = vec![
        City {
            name: "Dubai".to_string(),
            population: 3_137_000,
            country: "United Arab Emirates".to_string(),
        },
        City {
            name: "Lima".to_string(),
            population: 9_751_717,
            country: "Peru".to_string(),
        },
        City {
            name: "Puebla".to_string(),
            population: 2_704_000,
            country: "Mexico".to_string(),
        },
        City {
            name: "Reykjavik".to_string(),
            population: 120_000,
            country: "Iceland".to_string(),
        },
    ];
    println!("{:?}", cityes);
    //sort_cities(&mut cityes);
    let stat = Statistic::Max;
    //sort_by_statistic(&mut cityes, stat);
    let thread_handler = start_sorting_thread(cityes, stat);
    cityes = thread_handler.join().unwrap();
    println!("{:?}", cityes);

    let my_str = "Hello World!".to_string();
    let f = || {
        println!("{}", my_str);
        drop(my_str)
    };
    //f();
    //f();
    call_twice_once(f);

    let mut i = 0;
    let incr = || {
        i += 1;
        println!("ding! i is now: {}", i);
    };
    call_twice_mut(incr);

    let mut i = 0;
    call_twice_mut(|| i += 1);
    assert_eq!(i, 2);

    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;
    assert_eq!(add_y(copy_of_add_y(22)), 42);

    let mut greeting = String::from("Hello, ");
    let greet = move |name| {
        greeting.push_str(name);
        println!("{}", greeting);
    };
    greet.clone()("Alfred");
    greet.clone()("Bruce");
}

pub fn count_selected_cities(cities: &Vec<City>, test_fn: fn(&City) -> bool) -> usize {
    let mut count = 0;
    for city in cities {
        if test_fn(city) {
            count += 1;
        }
    }
    count
}

fn call_twice_once<F>(closure: F)
where
    F: FnOnce(),
{
    closure();
}

fn call_twice<F>(closure: F)
where
    F: Fn(),
{
    closure();
    closure();
}

fn call_twice_mut<F>(mut closure: F)
where
    F: FnMut(),
{
    closure();
    closure();
}
