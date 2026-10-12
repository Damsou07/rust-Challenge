// Stage 2 inverser les valeurs d'un tableau

fn main () {
    let mut x: [i32; 8] = [1, 2, 3, 5, 8, 9, 45, 2];
    reverse_tab(&mut x);
    println!("{:?}", x);

    let mut y: [i32; 7] = [1, 2, 3, 5, 8, 9, 45];
    reverse_tab(&mut y);
    println!("{:?}", y);

}

fn reverse_tab<T ,const N: usize>(tab: &mut[T; N]) {
    let mut j:usize = 0;
    let limit = N / 2;

    for i in (limit..N).rev() {
        tab.swap(i, j);
        j += 1;
    }
}


// .rev() -> permet de faire la boucle dans le sens inverse
// attention en rust : for i in a..b , on prend i = a et i_max = b-1 , 
    //b et toujours exclus et cette logique s'applique aussi pour la boucle inverse en rust