// Stage 3 rechercher un élement dans un tableau

fn main () {
    test_linear_search();
}


fn linear_search<T: PartialEq, const N: usize>(tab: &[T; N], target: &T,) -> Option<usize> {
    for i in 0..N {
        if target.eq(&tab[i]) {
            return Some(i);
        }
    }

    return None;
}


fn test_linear_search() {
    // Première position
    assert_eq!(linear_search(&[10, 20, 30], &10), Some(0));

    // Position intermédiaire
    assert_eq!(linear_search(&[10, 20, 30], &20), Some(1));

    // Dernière position
    assert_eq!(linear_search(&[10, 20, 30], &30), Some(2));

    // Élément absent
    assert_eq!(linear_search(&[10, 20, 30], &99), None);

    // Tableau vide
    let vide: [i32; 0] = [];
    assert_eq!(linear_search(&vide, &5), None);

    // Un seul élément
    assert_eq!(linear_search(&[42], &42), Some(0));

    // Première occurrence parmi plusieurs
    assert_eq!(linear_search(&[5, 8, 5, 8], &5), Some(0));

    // Nombres négatifs
    assert_eq!(linear_search(&[-10, -20, -30], &-20), Some(1));

    // Flottants
    assert_eq!(linear_search(&[1.5, 2.5, 3.5], &2.5), Some(1));

    // String : type non Copy
    let mots = [
        String::from("Rust"),
        String::from("Python"),
        String::from("Java"),
    ];
    assert_eq!(
        linear_search(&mots, &String::from("Python")),
        Some(1)
    );

    // NaN n'est pas égal à lui-même
    let flottants = [1.0, f64::NAN, 3.0];
    assert_eq!(linear_search(&flottants, &f64::NAN), None);

    println!("✅ Tous les 11 tests de linear_search ont réussi !");
}


// PartialEq -> le trait qui permet comparer par égalité deux valeurs (== ou !=)
// a.eq(b) -> permet de vérifier l'égalité (==)