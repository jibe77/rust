/*
Le mécanisme d'appartenance (ownership) en Rust est un ensemble de règles qui 
régissent la manière dont la mémoire est gérée. Il garantit la sécurité de 
la mémoire sans avoir recours à un ramasse-miettes (garbage collector). 
Ces règles sont vérifiées à la compilation et s'appliquent à tous les types de données.

Les règles de l'appartenance :
    * Chaque valeur a une variable qui en est le propriétaire.
    * Il ne peut y avoir qu'un seul propriétaire à la fois.
    * Quand le propriétaire sort du scope, la valeur est libérée.

Quand une variable en possède une autre, l'appartenance est transférée. 
Le propriétaire précédent ne peut plus utiliser la variable.
*/
fn main() {
    let s1 = String::from("hello"); // s1 est le propriétaire
    let s2 = s1;                    // L'appartenance est transférée à s2.
                                    // s1 n'est plus valide.
    println!("{}", s2);
    // println!("{}", s1); // Erreur de compilation : s1 a été déplacé

    // L'appartenance de `s` est transférée à `une_string`.
    let s = String::from("monde");
    prend_appartenance(s); 
    // println!("{}", s); // Erreur de compilation : `s` a été déplacé
    // le message d'erreur est le suivant : 
    //  - move occurs because `s` has type `String`, which does not implement the `Copy` trait

    let s = String::from("monde");
    let s_apres_appel = rend_appartenance(s); // `s_apres_appel` récupère l'appartenance
    println!("{}", s_apres_appel);

    // Ce mécanisme, bien que sécurisant, peut être fastidieux. 
    // C'est pourquoi Rust propose le concept de références et d'emprunt (borrowing) 
    // pour permettre à une variable d'accéder à une valeur sans en prendre l'appartenance.
}

fn prend_appartenance(une_string: String) { // une_string est maintenant propriétaire
    println!("{}", une_string);
} // une_string sort du scope, la mémoire est libérée.

fn rend_appartenance(une_string: String) -> String {
    une_string
}