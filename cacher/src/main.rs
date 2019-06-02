use cacher::Cacher;

fn main() {
    let mut int_cacher = Cacher::new(|x| x);
    let mut string_cacher = Cacher::new(|x| x);

    println!("a simple test for cacher 25 {}", int_cacher.value(25));
    println!("a simple test for cacher 'hello' {}", string_cacher.value("hello"));

    println!("generating first workout with intensity 10");
    cacher::generate_workout(10, 10);
    println!("generating second workout with intensity 25");
    cacher::generate_workout(25, 10);
    println!("generating third workout with a break :)");
    cacher::generate_workout(100, 3);
}