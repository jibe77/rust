fn main() {
    let val = Some(21);
    let val2: Option<i32> = None;

    println!("{:?}", val);
    println!("{:?}", val2);

    println!("{:?}", val.unwrap());
    println!("{:?}", val2.unwrap_or_default());
    println!("{:?}", val2.unwrap_or(-1));

    // or() va utiliser un autre option
    let val3 = val2.or(val);
    // or_else() prend une closure en paramètre
    println!("{:?}", val3.unwrap());

    let x = Some(32);
    let result = match x {
        Some(10) => Some(10 * 10),
        Some(32) => Some(32 * 32),
        _ => None
    };
    println!("{:?}", result);
}