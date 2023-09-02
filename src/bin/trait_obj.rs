trait Pet: Animal {
    fn speak(&self);
}

// Animal is a supertrait because Pet requires it to be implemented.
trait Animal {}

struct Cat {}
struct Dog {}

impl Pet for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

impl Animal for Cat {}

impl Pet for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

impl Animal for Dog {}

fn adopt_pet() -> impl Pet {
    // Since we are returning a single concrete type, we are using trait bounds.
    Cat {}
}

fn adopt_pet_with_preference(pref: &str) -> Box<dyn Pet> {
    if pref.to_lowercase().contains("cat") {
        // When returning multiple types, we have to use trait objects. Trait objects let us define
        // a type which implements a trait, without knowing what that type is at compile-time. This
        // is how Rust acheives polymorphism.
        // Trait objects are defined with the `dyn` keyword, which is short for 'dynamic dispatch',
        // and must be behind some type of pointer.
        return Box::new(Cat {});
    }
    Box::new(Dog {})
}

// cuddle() accepts a trait object as a parameter.
fn cuddle(_pet: &dyn Pet) {
    println!("Cuddling!");
}

fn main() {
    let my_pet = adopt_pet();
    cuddle(&my_pet);

    let my_dog = adopt_pet_with_preference("I want a dog!");

    // Use as_ref() to pass the pointer as a reference.
    cuddle(my_dog.as_ref());

    // Collection of different concrete types that implement the Pet trait.
    let pets: Vec<&dyn Pet> = vec![&my_pet, my_dog.as_ref()];
    pets.iter().for_each(|pet| pet.speak());
}
