use std::thread;
use std::time::Duration;

fn main() {
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculation slow...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    fn _expensive_function(num: u32) -> u32 {
        println!("calculation slow...");
        thread::sleep(Duration::from_secs(2));
        num
    }

    let mut expensive_result = Cacher::new(_expensive_function);

    expensive_result.value(10);
}

struct Cacher<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T: Fn(u32) -> u32> Cacher<T> 
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
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
