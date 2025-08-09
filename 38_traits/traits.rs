/*
 * Les traits en Rust sont des caractéristiques partagées que les types peuvent implémenter.
 */

// 1. Définition du trait Afficheur
// Ce trait déclare une seule méthode `afficher_alerte`.
trait Afficheur {
    fn afficher_alerte(&self);
}

// 2. Implémentation du trait pour une structure ArticleDeBlog
struct ArticleDeBlog {
    titre: String,
    auteur: String,
    contenu: String,
}

impl Afficheur for ArticleDeBlog {
    fn afficher_alerte(&self) {
        println!("Nouvel article de blog de {} : {}", self.auteur, self.titre);
    }
}

// 3. Implémentation du trait pour une structure Tweet
struct Tweet {
    utilisateur: String,
    contenu: String,
}

impl Afficheur for Tweet {
    fn afficher_alerte(&self) {
        println!("Nouveau tweet de {} : {}", self.utilisateur, self.contenu);
    }
}

// 4. Utilisation de la fonction générique `envoyer_alerte`
fn envoyer_alerte(item: &impl Afficheur) {
    item.afficher_alerte();
}

/**
 * La fonction envoyer_alerte n'a pas besoin de savoir si elle reçoit un ArticleDeBlog ou un Tweet. 
 * Elle sait seulement que l'objet a la capacité d'être "affichable" (via le trait Afficheur) et peut donc appeler la méthode afficher_alerte. C'est le principe du polymorphisme ad hoc, qui permet d'écrire du code générique et réutilisable.
 */
fn main() {
    let article = ArticleDeBlog {
        titre: String::from("Le pouvoir des traits en Rust"),
        auteur: String::from("Jean Dupont"),
        contenu: String::from("..."),
    };

    let tweet = Tweet {
        utilisateur: String::from("rust_dev"),
        contenu: String::from("J'adore les traits !"),
    };

    envoyer_alerte(&article); // Affiche : Nouvel article de blog de Jean Dupont : Le pouvoir des traits en Rust
    envoyer_alerte(&tweet);   // Affiche : Nouveau tweet de rust_dev : J'adore les traits !
}