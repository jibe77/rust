// cette ligne désacrive les avertissements du compilateurs pour les propriété de l'étudiant non utilisées.
#[allow(dead_code)]
// instruction spéciale que vous placez au-dessus d'une structure (ou d'une énumération) pour dire 
// au compilateur de générer automatiquement une implémentation du "trait" Debug.
// Il permet de formater une valeur de manière conviviale pour un développeur, de sorte qu'elle soit 
// facilement lisible et compréhensible lors de l'exécution du programme.
#[derive(Debug)]

/*
   struct (structure) est utilisé pour créer des types de données personnalisés. 
   Il vous permet de regrouper plusieurs valeurs, qui peuvent être de types différents, 
   sous un seul nom significatif.

   Pensez à une struct comme un plan ou un modèle pour créer des "objets" avec des 
   propriétés et des comportements spécifiques. C'est l'équivalent de classes dans 
   d'autres langages orientés objet, mais avec une approche plus axée sur les données.
*/
struct Students {
    rollno: u16,
    name: String,
    gender: bool
}

#[derive(Debug)]
struct Point (i16, i16);

#[derive(Debug)]
struct Line {
    top: Point, 
    bottom: Point
}

fn main() {
    let mut std = Students {
        rollno: 12,
        name: "jha".to_string(),
        gender: true
    };
    println!("{}", std.name);

    // std doit être muttable pour que cette ligne fonctionne.
    std.name = "jack".to_string();
    println!("{}", std.name);

    // cette ligne est permise par #[derive(Debug)]
    println!("{:?}", std);
    println!("{:#?}", std);

    // cette syntaxe permet la création d'un étudiant en récupérant des valeurs par défaut de std
    let std_bis = Students {
        rollno: 13,
        name: "ben".to_string(),
        ..std
    };
    println!("{:#?}", std_bis);

    let x = Point(4,12);
    println!("{:#?}", x);

    // Exemple de déstructuration d'une structure en Rust. 
    // C'est une façon de prendre une structure et d'en extraire 
    // les valeurs des champs pour les affecter à de nouvelles variables
    // Elle prend une instance de structure nommée std_bis
    // Elle en extrait les champs et les assigne à de nouvelles variables.
    // rollno: id : Cela signifie que la valeur du champ rollno de la 
    // structure std_bis est extraite et assignée à une nouvelle variable que vous avez nommée id.
    let Students{rollno: id, name, gender: ratio } = std_bis;
    println!("{}", ratio);

    // l'exemple suivant est un struct composé d'un struc
    let line = Line {
        top: x,
        bottom: Point(1,2)
    };
    println!("{:#?}", line);
}