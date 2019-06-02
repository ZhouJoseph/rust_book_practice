use std::collections::HashMap;
use std::hash;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub struct Cacher<T, U>
	where T: Fn(U) -> U,
		  U: Eq + hash::Hash + Copy,
{
	caculation: T,
	value: HashMap<U, U>,
}

impl<T, U> Cacher<T, U>
	where T: Fn(U) -> U,
		  U: Eq + hash::Hash + Copy,
{
	pub fn new(caculation: T) -> Cacher<T, U>{
		Cacher{
			caculation,
			value: HashMap::new(),
		}
	}

	pub fn value(&mut self, key: U) -> &U{
		if !self.value.contains_key(&key){
			let v = (self.caculation)(key);
			self.value.insert(v, v);
		}
		let res = match self.value.get(&key) {
			Some(v) => v,
			None => {
				panic!("sth went wrong with the insert");
			},
		};
		res
	}
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
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