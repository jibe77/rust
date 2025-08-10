mod pink;
mod registry;

// On déclare que le module 'calculs' fait partie de notre projet.
mod calculs;

fn main() {
    println!("Hello, world!");
    pink::say();
    registry::green::check();

    let result = calculs::add(5, 3);
    println!("5 + 3 = {}", result);

    // Cette ligne générerait une erreur car 'subtract' est une fonction privée.
    // let result_sub = calculs::subtract(5, 3);
}
/*
mod pink {
    pub fn say() {
        println!("Something");
    }
}
*/