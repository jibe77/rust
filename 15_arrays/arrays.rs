fn main() {
    // la taille du tableau est fixe et il doit être rempli sinon ça renvoit une erreur
    let one: [i32; 5] = [0, 1, 2, 3, 4];
    println!("my array : {:?}", one);
    println!("my array third element : {:?}", one[2]);

    // on doit passer par & pour obtenir une référence vers la tranche car c'est la façon 
    // la plus sûre en Rust de donner accès à des données (ici, une partie de votre tableau) 
    // à une autre fonction ou macro sans en transférer la propriété
    println!("my array first to third elements : {:?}", &one[0..2]);

    // on remarque qu'il y a 8 octets (32 bits) de différence entre les deux élements du tableau
    //      my array third element point  : 0x7ff7b1543824
    //      my array fourth element point : 0x7ff7b1543828
    // ces 32 correspondent à la taille d'un élément du tableau car c'est i32.
    println!("my array third element point : {:p}", &one[2]);
    println!("my array fourth element point : {:p}", &one[3]);

    
}