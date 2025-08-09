fn main() {
    let my_int = 123;
    println!("l'adresse mémoire du pointeur est {:p}.", &my_int);

    let num : i8 = 32;
    println!("la variable de mon entier {}.", num);

    // il faut faire attetion à utiliser un type adapté, par exemple u8 (unisgned de taille 8 bits) 
    // ne peut pas contenir des valeurs négatives. Cela empêche la compilation
    // let num_unsigned : u8 = -32;

    // pour les flottants on utilise le type f32 ou f64
    let my_float : f32 = 1.34234;
    println!("ma valeur flottante : {}", my_float);

    // ici les booléens
    let my_boolean : bool = true;
    println!("ma valeur booléene : {}", my_boolean);

    // ici les caractères
    let my_char : char = 'a';
    println!("mon caractère : {}.", my_char);

    // ici les chaines de caractères
    let my_string : String = "abc".to_string();
    println!("ma chaine de caractères : {}.", my_string);

    /*
    La décision de rendre les variables immutables par défaut est un choix de conception 
    fondamental en Rust, principalement motivé par la sécurité et la concurrence
    nommé Fearless Concurrency.
    Quand une variable est immutable, vous savez que sa valeur ne changera jamais après 
    son initialisation. Cela élimine toute une catégorie de bugs subtils où une variable 
    est modifiée involontairement dans une autre partie du code, rendant le programme 
    plus prévisible et plus facile à raisonner, notamment lors de la parallélisation.
    Lorsqu'on lit du code, on n'a pas besoin de se demander si une valeur a pu changer 
    entre deux lignes. On peut se fier à la valeur initiale.
     */
    // les valeurs sont immutables, donc il n'est pas permis de les modifier
    // num = 64;
    // pour les rendre modifiable, il faut utiliser le mot clé mut
    let mut num_mut : i8 = 16;
    println!("la variable de mon entier mutable avec sa valeur initiale {}.", num_mut);
    num_mut = 64;
    println!("la variable de mon entier mutable avec la valeur modifiée {}.", num_mut);
}