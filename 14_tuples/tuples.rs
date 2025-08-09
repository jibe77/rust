fn main() {
    let my_tuples = (1.2f32, 2.4f32, -4.5f32, "coucou");
    println!("print my tuple : {:?}", my_tuples);
    println!("je veux afficher le 4ème élément : {:?}", my_tuples.3);

    let tuple_of_tuple = (("Jha","ram","Singh"),(1,2,3),(2,3));
    let (names, three_num, _two_nums) = tuple_of_tuple;
    println!("{:?} {:?}:", names, three_num);
}