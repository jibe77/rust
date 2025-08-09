/**
 * Un des cas d'utilisation les plus importants est la création de structures de données récursives, 
 * comme les listes chaînées. Sans Box, le compilateur ne pourrait pas déterminer la taille de la structure.
 */
// `enum` de liste chaînée
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Création d'une liste chaînée
    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
            Box::new(List::Cons(3, 
                Box::new(List::Nil)
            ))
        ))
    );
}