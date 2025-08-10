// Par défaut, le module 'calculs' est privé.
// Mais sa visibilité est définie par son parent.

// Rendre cette fonction publique avec `pub`
// pour qu'elle puisse être utilisée en dehors du module 'calculs'.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Cette fonction reste privée au module 'calculs'.
fn subtract(a: i32, b: i32) -> i32 {
    a - b
}