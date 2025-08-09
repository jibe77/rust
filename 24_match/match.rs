#[warn(unreachable_patterns)]
fn main() {
    let num : u8 = 3;

    match num {
        0..=1   => println!("C'est la valeur zéro ou un."),
        2..4    => println!("Entre 2 et 4."),
        4..=255 => println!("Valeur par défaut.")
    }

    match num {
        1 => println!("c'est la valeur un."),
        _ => println!("Toutes les autres valeurs")
    }

    // matching avec un tuple
    let values = (3, 12, 9);
    match values {
        (3, b, c) => println!("le premier est 3 et le reste {} et {}.", b, c),
        (a, b, 9) => println!("le dernier est 9 et les premiers {} et {}.", a, b),
        (7, ..)   => println!("le premier est 7."),
        (.., 9)   => println!("le dernier est 9"),
        _         => println!("valeur par défaut")
    }

    // matching avec un tableau
    let arrays = [3, 9, 1];
    match arrays {
        [1, h, p]    => println!("le premier est 1 et le reste {} et {}.", h, p),
        [.., 6]      => println!("le dernier est 6."),
        [3, hg @ ..] => println!("le premier est 3 et le reste {:?}.", hg),
        _            => println!("valeur par défaut")
    }

    // matching avec un struc
    struct Foo {
        x: i8,
        y: (i8, bool)
    };

    let coo = Foo {
        x: 32,
        y: (1, false)
    };

    match coo {
        Foo{x: 1, y} => println!("x est 1 est y {:?}", y),
        _            => println!("valeur par défaut")
    }

    let Foo { x: val1, y: val2} = coo;
    println!("{val1} et {val2:?}");
}