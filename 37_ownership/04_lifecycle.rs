/*
En Rust, le type de retour &'static sur une fonction est une annotation de durée de vie qui 
signifie que la référence retournée est valide pendant toute la durée d'exécution du programme.

Durée de vie maximale : 'static est la durée de vie la plus longue possible en Rust. 
Une référence avec une durée de vie 'static est garantie d'être valide jusqu'à la fin de l'exécution du programme.

Aucune dépendance aux paramètres : Le fait de retourner &'static signifie que la valeur retournée ne 
dépend pas de la durée de vie des arguments de la fonction. Elle ne "prête" pas une référence à 
quelque chose qui a été passé en argument, mais retourne une référence à une donnée qui existe 
indépendamment de l'appel de la fonction.
*/
fn get_static_string() -> &'static str {
    // Cette chaîne de caractères est stockée dans le binaire
    // et a une durée de vie 'static.
    "Ceci est une chaîne de caractères statique." 
}

static MON_NOMBRE: i32 = 42;

fn get_static_number() -> &'static i32 {
    &MON_NOMBRE
}

fn main() {
    println!("ma chaine de caractères est {}", get_static_string());
    println!("mon nombre statique est {}", get_static_number());


}