fn main() {
    // cette variable n'est pas utilisé, donc il faut précéder le nom avec _ 
    // afin que le compilateur ne remonte pas d'avertissement.
    let _my_variable = "test";
    println!("hello");

    // exposant
    println!("1e4 : {}", 1e4);
    // opération sur les booléens
    println!("true && false = {}", true && false);
    // opération sur binaires
    println!("0b0101 | 0b1010 : {:b}", 0b0101 | 0b1010);
}