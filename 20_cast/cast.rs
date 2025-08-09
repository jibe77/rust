fn main() {
    // on créé un flottant
    let num =73.543_f32;
    println!("{}", num);

    let num_as_u8 = num as u8;
    println!("{}", num_as_u8);

    // le cast avec u8 est possible (mais pas avec i8)
    let num_as_char = num_as_u8 as char;
    println!("{}", num_as_char);
}