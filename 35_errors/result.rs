/*
le type Result<T, E> est le moyen standard et le plus sûr pour gérer les erreurs récupérables. 
Au lieu de lancer des exceptions, les fonctions qui peuvent échouer renvoient un Result, 
forçant ainsi le développeur à gérer explicitement le succès ou l'échec.

Result est une énumération générique qui a deux variantes :
    Ok(T) : Indique que l'opération a réussi. La valeur T est le résultat attendu.
    Err(E) : Indique que l'opération a échoué. La valeur E est l'erreur qui a eu lieu.

Le compilateur Rust vous oblige à gérer un Result. Vous ne pouvez pas utiliser directement 
la valeur d'un Result sans avoir vérifié si c'est un Ok ou un Err.
*/

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let val: Result<i32, String> = Ok(43); // i32 = result is successful
    let val2: Result<i32, String> = Err("There is a error".to_string()); // 132 = result is successful
    println! ("{:?}", val);
    println! ("{:?}", val.unwrap());
    println! ("{:?}", val2); 
    println! ("{:?}", val2.unwrap_err());

        let _f = File::open("hello.txt");

    // Utilisation de `match` pour gérer les deux cas
    let _f = match _f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("Fichier non trouvé. On va le créer.");
                File::create("hello.txt").unwrap()
            }
            other_error => panic!("Problème à l'ouverture du fichier : {:?}", other_error),
        },
    };
}