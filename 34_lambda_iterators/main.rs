fn main() {
    let nums = vec![1, 2, 3, 4, 76];
    for n in nums.iter() {
        println!("{:?}", n);
    }
    println!("{:?}", nums);

    // 1. On spécifie le type de la collection comme Vec<i32>, pas Vec<T>.
    // 2. On utilise l'opérateur de déréférencement * sur 'h'.
    let x: Vec<i32> = nums.iter().map(|h| h * 10).collect();
    // 3. On utilise le formateur de débogage '{:?}' pour afficher le vecteur.
    println!("{:?}", x);

    let pair: Vec<i32> = nums.clone().into_iter().filter(|a| *a % 2 == 0).collect();
    println!("{:?}", pair);

    let sum: i32 = nums.iter().sum();
    println!("{}", sum);
}