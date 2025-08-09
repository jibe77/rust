#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Une fonction statique (appelée avec `::`) pour créer une nouvelle instance
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Une méthode d'instance (appelée avec `.`) pour calculer l'aire
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Méthode pour modifier la largeur et la hauteur
    fn change(&mut self, new_width: u32, new_height: u32) {
        self.width = new_width;
        self.height = new_height;
    }
}


// le mot-clé impl est utilisé pour implémenter des fonctions pour une structure (struct).
// Ces fonctions sont appelées des méthodes.
impl Point {
    fn origin() -> Point {
        Point{x:0.0, y:0.0}
    }
    fn new(x:f64, y:f64) -> Point {
        return Point{x: x,y: y}
    }
}

fn main() {
    let p1 = Point{x:4.434, y:6.3432};
    println!("mon point 1 se situe à cet endroit {:#?}", p1);

    // l'opérateur :: est utilisé pour appeler des fonctions statiques
    // contrairement à . qui est utilisé pour appeler des méthodes d'instances
    let p2 = Point::origin();
    println!("mon point 2 se situe à cet endroit {:#?}", p2);

    let p3 = Point::new(5.43,7.342);
    println!("mon point 3 se situe à cet endroit {:#?}", p3);

    // Appel d'une fonction statique avec `::`
    let mut rect1 = Rectangle::new(30, 50);

    // Appel d'une méthode d'instance avec `.`
    println!("L'aire du rectangle est de {} pixels carrés.", rect1.area());

    rect1.change(60, 80);
    println!("Nouvelle aire : {}", rect1.area());

}