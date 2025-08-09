// cette instruction permet d'éviter les avertissements sur les entrées de 
// l'énumération non utilisées.
#[allow(dead_code)]
enum Light {
    Red,
    Yellow,
    Green
}

fn main() {
    // cette ligne permet d'éviter de rappeler Light:: devant chaque valeur utilisée
    // use Light::**;

    let ma_couleur = Light::Red;

    match ma_couleur {
        Light::Red => println!("rouge"),
        Light::Yellow => println!("jaune"),
        Light::Green => println!("vert")
    }
}