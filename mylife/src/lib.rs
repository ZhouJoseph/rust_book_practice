#[derive(Debug)]
struct Counter {
	thing: String,
}

impl Counter {
	fn new() -> Counter {
		Counter{thing: String::from("None")}
	}
}

impl Iterator for Counter {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {

		if self.thing == String::from("None") {
			self.thing = String::from("wake up");
		}else if self.thing == String::from("wake up") {
			self.thing = String::from("go to work");
		}else if self.thing == String::from("go to work") {
			self.thing = String::from("go back home");
		}else if self.thing == String::from("go back home"){
			self.thing = String::from("go to bed");
		}else if self.thing == String::from("go to bed"){
			self.thing = String::from("over");
		}

		if self.thing == String::from("over"){
			None
		}else{
			Some(self.thing.clone())
		}
	}
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(String::from("wake up")));
    assert_eq!(counter.next(), Some(String::from("go to work")));
    assert_eq!(counter.next(), Some(String::from("go back home")));
    assert_eq!(counter.next(), Some(String::from("go to bed")));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: Vec<String> = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(mut a, b)| {a.push_str(&b); a})
                                 .filter(|x| x.len() > 20)
                                 .collect();


    let res = sum.join("");
    assert_eq!(String::from("go to workgo back homego back homego to bed"), res);
}