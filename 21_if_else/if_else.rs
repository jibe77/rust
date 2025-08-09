fn main() {
    let age = 9;
    if age > 9 {
        println!("Oui, c'est supérieur à 9.");
    } else if age == 9 {
        println!("C'est égal à 9.");
    } else {
        println!("C'est inférieur à 9.");
    }

    let check = if age > 10 {
        println!("10");
        age * 2
    } else {
        println!("20");
        age * 3
    };
    println!("check = {}.", check);
}