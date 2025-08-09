/**
 * En Rust, un "box" (Box<T>) est un pointeur intelligent qui vous permet de stocker des données 
 * sur le tas (heap) plutôt que sur la pile (stack). 
 * C'est l'un des types de pointeurs intelligents les plus simples et les plus couramment utilisés.
 * 
 * L'exemple le plus simple consiste à déplacer des données de la pile vers le tas. 
 * C'est utile lorsque vous manipulez de grandes structures pour éviter les copies coûteuses.
 */
fn main() {
    // Cette grande structure est initialement sur la pile
    let grande_donnee = [0; 1000]; 

    // `Box::new` déplace la grande_donnee vers le tas.
    // La variable `b` est maintenant un pointeur de taille fixe sur la pile.
    let b = Box::new(grande_donnee); 

    println!("La boîte a été créée sur le tas.");
} // b est libérée ici, et la mémoire sur le tas est nettoyée automatiquement.