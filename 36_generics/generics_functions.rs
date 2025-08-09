/*
Une fonction générique est une fonction qui prend un ou plusieurs 
paramètres de type générique. 
Le type générique est spécifié entre chevrons (<>) après le nom de la fonction.

Dans cet exemple, T est le paramètre de type générique. 
La "contrainte de trait" (T: PartialOrd) est cruciale ici. 
Elle indique que T doit implémenter le trait PartialOrd, 
car nous utilisons l'opérateur de comparaison >. Sans cette contrainte, 
le compilateur ne saurait pas comment comparer des valeurs de type T.
*/
fn trouver_le_plus_grand<T: PartialOrd>(liste: &[T]) -> &T {
    let mut plus_grand = &liste[0];
    for item in liste {
        if item > plus_grand {
            plus_grand = item;
        }
    }
    plus_grand
}

fn main() {
    let nombre_liste = vec![34, 50, 25, 100, 65];
    let resultat_nombre = trouver_le_plus_grand(&nombre_liste);
    println!("Le plus grand nombre est : {}", resultat_nombre);

    let char_liste = vec!['y', 'm', 'a', 'q'];
    let resultat_char = trouver_le_plus_grand(&char_liste);
    println!("Le plus grand caractère est : {}", resultat_char);
}

/* Pour des contraintes plus complexes, la clause where est souvent plus lisible.
fn trouver_le_plus_grand<T>(liste: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut plus_grand = &liste[0];
    for item in liste {
        if item > plus_grand {
            plus_grand = item;
        }
    }
    plus_grand
}*/