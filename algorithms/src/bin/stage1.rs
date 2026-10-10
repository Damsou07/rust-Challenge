// Stage 1 trouver max dans un liste

fn main() {
    let x: [i32; 8] = [1, 2, 3, 5, 8, 9, 45, 2];
    let tab_vide = [];
    let tab_negatif = [-23, -45, -89, -45];
    //let tab_flotant = [0.45, 0.4, 2.45, 6.56];

    // test de la fct
    println!("Max de la liste -> {:?}", find_max(&x));
    println!("tableau vide -> {:?}", find_max(&tab_vide));
    println!("Tableau Négatif -> {:?}", find_max(&tab_negatif));
    //println!("Tableau des Flotant -> {:?}", find_max(&tab_flotant));
}

fn find_max<const N: usize>(tab: &[i32; N]) -> Option<i32> {
    let mut max: i32;
    if N > 0 {
        max = tab[0];
    } else {return None}

    for i in 1..N {
        if max < tab[i] {
            max = tab[i];
        }
    }
    return Some(max);
}

// Notions apprises (*T->Type) ->
    // {:?} -> permet d'afficher les None et les Some(T)
    // Option<T> permet de déclarer un variable qui peut potentiel retourner None
    // usize -> est un entier non signé adapté aux indices et aux tailles 
    // Some(T) -> construit une valeur de type Option<T> (enveloppe dans un Option<T>)
    // &T -> fait référence, on ne passe pas le tableau dans la fonction mais sa référence

