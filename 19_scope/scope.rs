/*
Le "frozing" est une mesure de sécurité essentielle pour le modèle de possession et 
d'emprunt (ownership and borrowing) de Rust. Il garantit qu'il n'y a pas de conflit d'accès à la mémoire.

Imaginez que vous ayez une référence mutable à une variable (y dans l'exemple ci-dessus) et que 
vous utilisiez la variable originale (x) en même temps. Cela pourrait créer des 
"data races" (conditions de concurrence) et des incohérences, où y et x tenteraient d'accéder ou 
de modifier la même donnée de manière non synchronisée. Le compilateur Rust empêche cela en gelant 
la variable d'origine tant qu'une référence mutable existe.

Dans cet exemple, a est gelée après la ligne let b = &mut a;. 
Le compilateur ne vous laissera pas utiliser a tant que la référence mutable b existe. 
Une fois que b a quitté le "scope", le gel est levé, et vous pouvez à nouveau utiliser a.

Le concept de "frozing" est donc un mécanisme de sécurité qui protège contre les accès 
concurrents à des données mutables, renforçant la promesse de sûreté de la mémoire de Rust.
*/
fn main() {
    let mut a = String::from("Hello");

    let b = &mut a; // 'a' est maintenant gelée.
    
    // Si on décommente la ligne ci-dessous, on obtient une erreur de compilation !
    // println!("{}", a);

    b.push_str(", world!");

    // 'b' sort du scope ici. 'a' est dégelée.
    
    // Maintenant, c'est valide d'utiliser 'a'.
    println!("{}", a); // Affiche "Hello, world!"
}