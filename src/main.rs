use std::thread;
use std::time::Duration;

fn main() {
    let intensity_user = 9;
    let random_number = 8;

    create_workout(intensity_user, random_number);
}

fn create_workout(intensity: u32, random_number: u32) {
    let closure = |num| {
        println!("loading... slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    if intensity < 25 {
        println!("do {} push-ups", closure(intensity));
        println!("Next, do {} squats", closure(intensity));
    } else {
        if random_number == 3 {
            println!("take a break today");
        } else {
            println!("Today you have jogging {} minute", closure(intensity));
        }
    }
}
