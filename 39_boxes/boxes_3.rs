/**
 * Utiliser des "trait objects" pour le polymorphisme dynamique

 * Ce cas d'utilisation permet de stocker différents types de données dans la même collection ou de les 
 * passer à une fonction, à condition qu'ils implémentent le même trait. Cela est rendu possible par 
 * le Box<dyn Trait>.
 */

trait Dessinable {
    fn dessiner(&self);
}

struct Cercle;
impl Dessinable for Cercle {
    fn dessiner(&self) {
        println!("Dessine un cercle");
    }
}

struct Carre;
impl Dessinable for Carre {
    fn dessiner(&self) {
        println!("Dessine un carré");
    }
}

/**
 * Vec<Box<dyn Dessinable>> est un vecteur qui peut contenir des pointeurs vers n'importe quel 
 * type qui implémente le trait Dessinable. Sans Box, cela ne serait pas possible car les 
 * types Cercle et Carre n'ont pas la même taille et ne peuvent pas être stockés directement 
 * dans un vecteur hétérogène.
 */
fn main() {
    // `figures` est un vecteur de "trait objects".
    // Chaque élément a une taille différente, mais ils sont tous
    // stockés comme un Box de taille uniforme sur la pile.
    let figures: Vec<Box<dyn Dessinable>> = vec![
        Box::new(Cercle),
        Box::new(Carre),
    ];

    // On peut maintenant itérer sur la collection et appeler la méthode `dessiner`
    // de manière polymorphe.
    for figure in figures {
        figure.dessiner();
    }
}