// String est une chaîne de caractères possédée. 
// Elle est allouée sur le tas (heap), ce qui la rend dynamique, mutable et redimensionnable.
// &str est une tranche de chaîne de caractères (string slice). 
// C'est une référence non possédée, immuable et de taille fixe vers une séquence de données UTF-8.
fn main() {
    // exemple de code utilisant &str, on voit qu'ils ont tous les deux la même référence
    let word = "Hello";
    let word2 = word;
    println!("{:p} {:p}", word, word2);

    // example de code utilisant string
    let mut word = String::from("Hello");
    println!("{:p}", &word);
    word = "Name".to_string(); // &str
    println!("{:p}", &word);
    word.push_str(" is Jha.");
    println!("{:p}", &word);
    word.insert_str(7, "jfldsjfdlfj");
    println!("{:p}", &word);
    println!("{}", word);

}