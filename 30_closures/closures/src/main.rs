/*
Une closure est un type de fonction qui peut, en plus de prendre 
des paramètres, capturer et utiliser les variables du contexte 
dans lequel elle a été créée. Elle "se souvient" de cet environnement.

Les closures sont au coeur de la programmation fonctionnelle et des lambdas.

En tant que développeur, vous utilisez le plus souvent la syntaxe d'une 
expression lambda (x -> x + 1 en Java, |x| x + 1 en Rust, (x) => x + 1 en 
JavaScript) pour écrire votre code. Ce faisant, le compilateur ou l'interpréteur 
crée automatiquement une closure pour vous en arrière-plan, en s'assurant que 
toutes les variables dont la lambda a besoin de son environnement sont 
correctement capturées et accessibles.
*/
fn main() {
    let num: i32 = 5;

    // Une closure qui capture la variable 'num' de son environnement
    let add_num = |x: i32| x + num;

    let result: i32 = add_num(10); // La closure utilise la valeur de 'num'
    println!("{}", result); // Affiche 15

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let limit = 5;
 
    // Le filtre utilise la closure pour sa logique
    // L'opérateur de déréférencement * permet d'accéder à la valeur pointée par cette référence. 
    // Sans cet astérisque, tu comparerais une référence à un nombre, ce qui n'a pas de sens en Rust.
    let greater_than_5: Vec<i32> = numbers.into_iter().filter(|x| *x > limit).collect();

    println!("{:?}", greater_than_5); // Affiche [6, 7, 8, 9, 10, 11, 12]

}