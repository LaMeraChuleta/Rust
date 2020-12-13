use std::thread;
use std::time::Duration;

struct Cacher<T>
    where
    T: Fn(u32) -> u32,{
        calculation: T,
        value: Option<u32>,
    }

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
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
fn generate_workout(intensity: u32, random_number: u32) {
    
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    //Aprendiendo Iteradores
    let v = vec![1,2,3,4,5,6,7,8,9];
    let v = v.into_iter()
        .filter(|x| x > &5)
        .map(|x| x * 2)
        .collect::<Vec<_>>();
    println!("{:?}",v);
    
    let t = vec![1,2,3,4,5,6,7];
    let t: Vec<i32> = t.into_iter()
        .filter_map(|x| {
            match x < 5 {
                true => Some(x),
                _ => None,
            }
        })        
        .collect();
    println!("{:?}",t);

    let s = vec![1,2,3,4,5,6,7,21,34,43];
    let s = s.into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    println!("{:?}",s);
}