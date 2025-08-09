fn main() {
    let mut score = 0u8;
    loop {
        score += 1;

        if score == 5 {
            println!("passe la balle");
            continue;
        }
        println!("le score est {}", score);

        if score == 9 {
            println!("la partie est termine.");
            break;
        }
    }

    let results = loop {
        score += 1;
        if score == 10 {
            break score * 4;
        }
    };
    println!("{}",results);

    let mut num = 10;
    while num <= 20 {
        num += 1;
        if num == 13 {
            break;
        }
        println!("{}",num);
    }

    for n in 100..=105 {
        println!("{}", n);
    }
    
    println!("fin du programme");
}