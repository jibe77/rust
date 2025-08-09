// le fonction main démarre le programme
fn main() {
    /*  En Rust, le point d'exclamation ! après le nom d'une fonction 
     comme dans println! indique qu'il s'agit d'une macro et non 
     d'une fonction régulière. */
    println!("Hello");
    println!("Salut, mon nom est {} !", "JB");
    println!("et j'ai {age} ans.", age = 42);
    println!("Mon fils s'appelle {1} et il a {0} ans.", 12, "Théodore");
    println!("On peut aussi écrire {0} en hexadecimal {0:X} et en octal {0:o}", 123456);
    println!("On peut aussi écrire {0:0>10} en définissant un left padding (remplissage à gauche) avec des 0.", 123456);
    println!("On peut aussi écrire {0:<10} en définissant un right padding (remplissage à droite) avec des espaces.", 123456);
}