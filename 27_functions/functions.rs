fn main() {
    ma_fonction(12);
    println!("ma fonction module {}.", ma_fonction_modulo(27));
}

fn ma_fonction(val: u8) {
    for x in 1..=val {
        println!("{}", x);
    }
}

// -> boolean : C'est le type de la valeur que la fonction renvoie.
fn ma_fonction_modulo(val: u8) -> bool {
    if val % 3 == 0 {
        return true;
    }
    return false;
}