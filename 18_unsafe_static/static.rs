//  Une variable statique est un emplacement 
// de stockage global et unique qui existe pendant toute la vie du programme
// Pour les données qui sont partagées et accessibles par n'importe quelle partie du 
// programme, comme un compteur de requêtes, un cache, ou des données de configuration globales.
static mut MY_COUNTER : i32 = 0;

fn main(){
    // les constantes sont calculées au moment de la compilation
    const NUMBER: i32 = 45;
    println!("Ma constante : {}.", NUMBER);

    // mutable statics can be mutated by multiple threads: 
    // aliasing violations or data races will cause undefined behavior
    unsafe{
        MY_COUNTER += 1;
        println!("my counter : {}", MY_COUNTER);
    }
}