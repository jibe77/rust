/**
 * Pour résoudre ce problème, vous devez indiquer au compilateur que la durée de vie de 
 * la référence retournée est liée à la durée de vie des références passées en arguments. 
 * Voici comment on utilise l'annotation 'a' :
 */
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");

    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("La plus longue est: {}", result);
    }
}