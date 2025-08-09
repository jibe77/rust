/*
Vous pouvez définir des structs qui peuvent contenir des champs de types génériques. 
C'est très utile pour des structures de données comme Option ou Result, 
qui sont nativement génériques en Rust. 
*/
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // Un Point avec des i32
    let point_entier = Point { x: 5, y: 10 };
    println!("Point entier : ({}, {})", point_entier.x, point_entier.y);

    // Un Point avec des f64
    let point_flottant = Point { x: 1.0, y: 4.0 };
    println!("Point flottant : ({}, {})", point_flottant.x, point_flottant.y);
}