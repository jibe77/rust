fn main() {
    // Un panic est utilisé pour gérer les erreurs 
    // qui indiquent une situation irrécupérable, généralement un bug dans le code.
    panic!("Problem during execution ! Panic !");

    //  Le programme remonte la pile d'appels (call stack), libérant les ressources 
    // de chaque fonction au fur et à mesure. C'est le comportement par défaut de Rust 
    // et il est comparable au mécanisme des exceptions en C++ ou en Java. C'est sûr, mais cela a un coût en performance.


}