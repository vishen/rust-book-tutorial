use std::thread;
use std::time::Duration;

fn main() {
    println!("Iterators and closures");
    closures();
}

fn closures() {
   let simulated_user_specified_value = 10;
   let simulated_random_number = 7;

   generate_workout(
       simulated_user_specified_value,
       simulated_random_number,
    ); 
}

struct Catcher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Catcher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Catcher<T> {
        Catcher{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Catcher::new(|num| {
        simulated_expensive_calculation(num)
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break");
    } else {
        println!("Today, run for {} minutes", expensive_result.value(intensity));
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
