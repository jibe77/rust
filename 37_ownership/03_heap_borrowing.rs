/*
Le tas (heap) est une zone de mémoire plus flexible. 
Les données peuvent être stockées dans n'importe quel ordre.

L'allocation sur le tas est plus lente car le système d'exploitation 
doit trouver un espace libre de la bonne taille.

Elle est utilisée pour stocker des données dont la taille est inconnue 
à la compilation ou qui peut varier, comme les chaînes de caractères 
redimensionnables (String), les vecteurs (Vec) et les collections.

Pour les types de données sur le heap, comme un String, 
c'est la sémantique de déplacement (move) qui s'applique.
Une affectation transfère l'appartenance et invalide l'ancienne variable, 
ce qui empêche une double libération de la même zone de mémoire.
*/
fn main() {
    let s1 = String::from("hello"); // s1 est un pointeur sur le tas
    let s2 = s1; // Transfert d'appartenance

    println!("{}", s2);
    // println!("{}", s1); // Erreur de compilation : s1 est invalide

    let s3 = String::from("hello"); // s1 est un pointeur sur le tas
    let s4 = s3.clone(); // on clone afin de garder l'appartenance

    println!("{}", s4);
    println!("{}", s3);

    /*
    Le système d'emprunt (borrowing) en Rust est un mécanisme qui permet de référencer 
    une valeur sans en prendre l'appartenance. C'est une extension des règles d'appartenance 
    qui garantit la sécurité de la mémoire en empêchant les pointeurs invalides (dangling pointers) 
    ou les modifications concurrentes.

    Le borrowing est régi par les règles suivantes :
        Un seul emprunt mutable ou plusieurs emprunts immutables :
            * Vous pouvez avoir une seule référence mutable (&mut T) vers une valeur à un moment donné. Une référence mutable permet de modifier la valeur d'origine.
              Vous pouvez avoir plusieurs références immutables (&T) vers une valeur, 
              mais pas de référence mutable en même temps. 
              Une référence immutable ne permet que la lecture de la valeur.

            * L'emprunt ne peut pas survivre à la valeur :
              Le compilateur s'assure que la durée de vie de la référence est plus courte ou 
              égale à la durée de vie de la valeur qu'elle pointe.

    */

    // Ici, s est le propriétaire de la String. 
    // On emprunte s pour le passer à la fonction calculer_longueur, qui ne fait que lire la valeur. 
    // La variable s reste valide après l'appel de la fonction.
    let s = String::from("hello");
    let len = calculer_longueur(&s); // On passe une référence (&s)
    
    println!("La longueur de '{}' est {}.", s, len); // `s` est toujours utilisable

    // Ici, on a besoin de modifier la String dans la fonction ajouter_suffixe. 
    // On doit donc passer une référence mutable (&mut s).
    let mut s = String::from("hello"); // La variable s doit être mutable
    ajouter_suffixe(&mut s); // On passe une référence mutable
    
    println!("{}", s); // Affiche "hello world"

    /*
    Le compilateur refuse de compiler car `r3` est un emprunt mutable alors que 
    `r1` et `r2` sont toujours "actifs".

    let mut s = String::from("hello");
    let r1 = &s; // Emprunt immutable
    let r2 = &s; // Emprunt immutable
    let r3 = &mut s; // Erreur ! Emprunt mutable
    println!("{}, {}, {}", r1, r2, r3); 
    */

    // En Rust, le mot-clé ref est utilisé dans les patterns de déstructuration 
    // (par exemple, dans les match ou les let complexes) pour créer une référence 
    // à une valeur sans en prendre l'appartenance.
    let greetings: Option<String> = Some(String::from("hi there"));
    match greetings {
        Some(ref msg) => println!(" {}", msg),
        None => println!(" Nothing to greet"),
    }
    // Ceci est désormais valide, car on a simplement emprunté la valeur.
    println!("Greetings? {:?}", greetings);
    // Sans ref, on a une erreur : `greetings` a été déplacé dans le `match` et n'est plus utilisable.
}

fn calculer_longueur(s: &String) -> usize {
    s.len()
}

fn ajouter_suffixe(s: &mut String) {
    s.push_str(" world");
}