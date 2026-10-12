// Stage 1 trouver max dans un liste

use std::cmp::Ordering;

fn main() {
    test_find_max();
}

fn find_max<T: PartialOrd + Copy ,const N: usize>(tab: &[T; N]) -> Option<T> {
    let mut max: T;
    let mut j: usize = 0;

    if N > 0 {
        while tab[j].partial_cmp(&tab[j]).is_none() {
            j += 1;
            if j == N { return None;}
        }
        max = tab[j];
    } else {return None}

    for i in j+1..N {
        if max.partial_cmp(&tab[i]) == Some(Ordering::Less) {
            max = tab[i];
        }
    }
    return Some(max);
}

fn test_find_max() {
    // 1. Tableau classique
    let x: [i32; 8] = [1, 2, 3, 5, 8, 9, 45, 2];
    assert_eq!(find_max(&x), Some(45));

    // 2. Tableau vide
    let tab_vide: [i32; 0] = [];
    assert_eq!(find_max(&tab_vide), None);

    // 3. Tableau de nombres négatifs
    let tab_negatif = [-23, -45, -89, -45];
    assert_eq!(find_max(&tab_negatif), Some(-23));

    // 4. Tableau de flottants
    let tab_flotant = [0.45, 0.4, 2.45, 6.56];
    assert_eq!(find_max(&tab_flotant), Some(6.56));

    // 5. Un seul élément
    let unique = [42];
    assert_eq!(find_max(&unique), Some(42));

    // 6. Tous les éléments identiques
    let identiques = [7, 7, 7, 7];
    assert_eq!(find_max(&identiques), Some(7));

    // 7. Mélange de nombres négatifs, positifs et zéro
    let melange = [-100, 0, 50, -25, 100];
    assert_eq!(find_max(&melange), Some(100));

    // 8. Maximum au début
    let max_debut = [99, 5, 3, 1];
    assert_eq!(find_max(&max_debut), Some(99));

    // 9. Maximum à la fin
    let max_fin = [1, 3, 5, 99];
    assert_eq!(find_max(&max_fin), Some(99));

    // 10. Limites des entiers
    let extremes = [i32::MIN, 0, i32::MAX];
    assert_eq!(find_max(&extremes), Some(i32::MAX));

    // 11. Flottants négatifs
    let flottants_negatifs = [-1.5, -0.25, -9.8];
    assert_eq!(find_max(&flottants_negatifs), Some(-0.25));

    // 12. Entiers 64 bits
    let grands_entiers: [i64; 3] = [10, 9_000_000_000, 100];
    assert_eq!(find_max(&grands_entiers), Some(9_000_000_000));

    // NaN au début
    let nan_debut = [f64::NAN, 5.0, 10.0];
    assert_eq!(find_max(&nan_debut), Some(10.0));

    // NaN au milieu
    let nan_milieu = [5.0, f64::NAN, 10.0];
    assert_eq!(find_max(&nan_milieu), Some(10.0));

    // NaN à la fin
    let nan_fin = [5.0, 10.0, f64::NAN];
    assert_eq!(find_max(&nan_fin), Some(10.0));

    // Uniquement des NaN
    let nan_uniquement = [f64::NAN, f64::NAN];
    assert_eq!(find_max(&nan_uniquement), None);

    println!("Tous les tests de find_max ont réussi !");
}

// Notions apprises ->
    // :T -> Le type générique s'adapte et est aptimisé en fonction de la valeur , (type de variables abstrait)
    // {:?} -> permet d'afficher les None et les Some(T), les enum objets et autres
    // Option<T> permet de déclarer un variable qui peut potentiel retourner None
    // usize -> entier postif de taille non définis
    // Some(T) -> construit une valeur de type Option<T> (enveloppe dans un Option<T>)
    // &T -> fait référence, dans find_max : on ne passe pas le tableau dans la fonction mais sa référence
    // Les NaN ils peuvent apparaître suite à une erreur mathématique sur les flotants par exemple 0.0 / 0.0
    // T : Copy -> copy le tableau dans find_max c'est utile car on passe déja une référence dans la fonction donc quand on fait
        // max = tab[i], il faut pouvoir accéder au tableau, donc on utilise un copy, petite précision la c'est parce qu'on utilise un tableau génerique [T, N],
        // avec un type définis comme i32 rust copie juste la valeur
    // T: PartialOrd -> permet de comparer (>, <, >=, <=) les générique T et gère le case pour les entiers et décimaux, 'Ord' pour seulement les entiers
    // is_none() -> compare deux élements et définis si le résulat est none ou non (*pratique pour détecter les Nan)
    // is_some() -> revient à faire !is_none donc son contraire (en gros true si ça renvoie pas un none) permet d'éviter la double négation
     
/*
Petit point sur les performances

Mon algo vs max rust native : ------
    Tableau : 1000000 éléments
    Maximum : Some(100001)
    Médiane sur 30 mesures :
    find_max perso : 0.441 ms
    Rust natif     : 0.105 ms
    Ratio perso / natif : 4.22x

Pour comparer en python voila la lenteur de python : ------
    Tableau : 1 000 000 éléments
    Maximum : 100001
    Médiane sur 5 mesures :
    find_max perso : 24.874 ms
    Python natif   : 7.267 ms
    Ratio perso / natif : 3.42x
*/
