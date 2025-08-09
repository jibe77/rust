/*
La pile (stack) est une zone de mémoire où les données sont stockées de manière linéaire et ordonnée. 
Les données sont ajoutées (push) et retirées (pop) en respectant le principe LIFO (Last-In, First-Out).

L'allocation sur la pile est rapide et performante, car le compilateur connaît la taille exacte de toutes les données.

Elle est utilisée pour stocker des variables dont la taille est connue à la compilation, 
comme les entiers (i32), les flottants (f64), les caractères, les booléens et les références.

Pour les types de données sur la stack, comme un i32, une simple copie de la valeur est effectuée 
lors d'une affectation. L'ancienne variable reste valide. C'est ce qu'on appelle la sémantique de copie (copy).

Lorsque la ligne let y = x; est exécutée, Rust ne transfère pas l'appartenance (il n'y a pas de "déplacement"). 
Au lieu de cela, il effectue une copie bit à bit de la valeur de x et la place dans la nouvelle variable y.
*/
fn main() {
    let mut x = 5;
    let y = x; // `y` est une copie de `x`
    x = 10;

    println!("x = {}", x);
    println!("y = {}", y); // la valeur de y est celle de x au moment de la copie

    let _a: &i32;
    {
        let b: i32 = 10;
        _a = &b;
    }
    // ce code ne fonctionne pas car la valeur référencée n'est plus disponible (hors-scope)
    //println!("{}", _a);

    let a: &str = call();
    println!("{a}");

    let x: i32 = 10;
    let y: i32 = 90;

    let c: &i32 = call_me(&x, &y);

    println!({c});
}