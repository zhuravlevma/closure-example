use std::collections::HashSet;
use std::thread;
use std::time::Duration;

fn main() {
    let intensity_user = 9;
    let random_number = 8;

    create_workout(intensity_user, random_number);
}

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    values: HashSet<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            values: HashSet::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        for value in &self.values {
            if *value == arg {
                return arg;
            }
        }
        let value = (self.calculation)(arg);
        self.values.insert(value);
        value
    }
}

fn create_workout(intensity: u32, random_number: u32) {
    let mut cache = Cache::new(|num| {
        println!("loading... slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("do {} push-ups", cache.value(intensity));
        println!("Next, do {} squats", cache.value(intensity));
        println!("Next, do {} pull-ups", cache.value(22));
    } else if random_number == 3 {
        println!("take a break today");
    } else {
        println!("Today you have jogging {} minute", cache.value(intensity));
    }
}
