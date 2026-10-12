// Stage 2 inverser les valeurs d'un tableau

fn main () {
    test_reverse_tab();
}

fn reverse_tab<T ,const N: usize>(tab: &mut[T; N]) {
    let mut j:usize = 0;
    let limit = N / 2;

    for i in (limit..N).rev() {
        tab.swap(i, j);
        j += 1;
    }
}


fn test_reverse_tab() {
    // 1. Tableau classique de taille paire
    let mut pairs = [1, 2, 3, 4, 5, 6];
    reverse_tab(&mut pairs);
    assert_eq!(pairs, [6, 5, 4, 3, 2, 1]);

    // 2. Tableau de taille impaire
    let mut impairs = [1, 2, 3, 4, 5];
    reverse_tab(&mut impairs);
    assert_eq!(impairs, [5, 4, 3, 2, 1]);

    // 3. Tableau vide
    let mut vide: [i32; 0] = [];
    reverse_tab(&mut vide);
    assert_eq!(vide, []);

    // 4. Un seul élément
    let mut unique = [42];
    reverse_tab(&mut unique);
    assert_eq!(unique, [42]);

    // 5. Deux éléments
    let mut deux = [10, 20];
    reverse_tab(&mut deux);
    assert_eq!(deux, [20, 10]);

    // 6. Nombres négatifs et zéro
    let mut negatifs = [-10, 0, -30, 5];
    reverse_tab(&mut negatifs);
    assert_eq!(negatifs, [5, -30, 0, -10]);

    // 7. Éléments identiques
    let mut identiques = [7, 7, 7, 7, 7];
    reverse_tab(&mut identiques);
    assert_eq!(identiques, [7, 7, 7, 7, 7]);

    // 8. Nombres flottants
    let mut flottants = [1.5, -2.75, 0.0, 9.25];
    reverse_tab(&mut flottants);
    assert_eq!(flottants, [9.25, 0.0, -2.75, 1.5]);

    // 9. Chaînes de caractères empruntées
    let mut mots = ["Rust", "est", "puissant"];
    reverse_tab(&mut mots);
    assert_eq!(mots, ["puissant", "est", "Rust"]);

    // 10. String : type qui n'implémente pas Copy
    let mut textes = [
        String::from("Bonjour"),
        String::from("mon"),
        String::from("reuf"),
    ];
    reverse_tab(&mut textes);
    assert_eq!(textes, ["reuf", "mon", "Bonjour"]);

    // 11. Booléens
    let mut booleens = [true, false, false, true, false];
    reverse_tab(&mut booleens);
    assert_eq!(booleens, [false, true, false, false, true]);

    // 12. Valeurs extrêmes
    let mut extremes = [i32::MIN, 0, i32::MAX];
    reverse_tab(&mut extremes);
    assert_eq!(extremes, [i32::MAX, 0, i32::MIN]);

    // 13. NaN : vérifier le déplacement, pas l'égalité
    let mut avec_nan = [1.0_f64, f64::NAN, 3.0];
    reverse_tab(&mut avec_nan);
    assert_eq!(avec_nan[0], 3.0);
    assert!(avec_nan[1].is_nan());
    assert_eq!(avec_nan[2], 1.0);

    // 14. Double inversion : retrouver le tableau initial
    let original = [11, 22, 33, 44, 55, 66, 77];
    let mut double_inversion = original;
    reverse_tab(&mut double_inversion);
    reverse_tab(&mut double_inversion);
    assert_eq!(double_inversion, original);

    // 15. Grand tableau : 10 000 éléments
    let mut grand = std::array::from_fn::<i32, 10_000, _>(|i| i as i32);
    reverse_tab(&mut grand);

    for i in 0..grand.len() {
        assert_eq!(grand[i], (grand.len() - 1 - i) as i32);
    }

    println!("✅ Tous les 15 tests de reverse_tab ont réussi !");
}



// .rev() -> permet de faire la boucle dans le sens inverse
// attention en rust : for i in a..b , on prend i = a et i_max = b-1 , 
    //b et toujours exclus et cette logique s'applique aussi pour la boucle inverse en rust
// tableau.swap(a, b) -> permet de permutter deux élements d'un même tableau
// (& mut T) -> permet de modifier la réference passé en paramètre de la fonction, donc pas besoin de revoyer la même variable