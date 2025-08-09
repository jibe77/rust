/* 
En Rust, l'apostrophe (') a une signification très spécifique et est principalement 
utilisée pour les durées de vie (lifetimes).

C'est l'utilisation la plus courante. En Rust, le compilateur, à travers un mécanisme 
appelé le "borrow checker", s'assure que les références pointent toujours vers des 
données valides. Une durée de vie est un concept qui permet de nommer et de lier 
la durée de validité d'une référence à celle de la donnée qu'elle pointe.

L'apostrophe est le caractère syntaxique qui permet de nommer une durée de vie. 

Les références ne doivent pas survivre à la donnée : Une référence (emprunt) doit toujours 
avoir une durée de vie (lifetime) plus courte ou égale à celle de la donnée qu'elle 
référence. Le borrow checker utilise les annotations de durée de vie ('a, 'b', 'static') 
pour vérifier cette règle à la compilation.

l'exemple classique pour illustrer les durées de vie (lifetimes) est la fonction qui trouve la 
plus longue de deux "slices" de chaînes de caractères.
*/
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/**
 * Si vous essayez de compiler ce code, le compilateur vous donnera une erreur. 
 * Il ne sait pas si la référence retournée par la fonction longest (result) doit avoir 
 * la même durée de vie que string1 ou que string2. Il ne peut pas garantir que result 
 * ne survivra pas à l'une de ces deux variables. C'est là que les annotations de durée
 *  de vie entrent en jeu.
 * 
 */
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("La plus longue est: {}", result);
}