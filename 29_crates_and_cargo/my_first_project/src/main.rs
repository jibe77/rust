// pour créer le projet
// % cargo new my_first_project

// pour ajouter uuid
// % cargo add uuid --features v4

// lancement de l'application
// % cargo run

use uuid::Uuid;

fn main() {
    println!("Hello, {}!", Uuid::new_v4());
}
