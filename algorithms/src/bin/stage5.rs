// Trie un tableau dans l'ordre croissant

use std::cmp::Ordering;

fn main() {
    let mut x = [2, 2, 8, 56, 9, 4, 20, 1, 7];
    selection_sort(&mut x);
    println!("{:?}", x )


}


fn selection_sort<T: Ord, const N: usize>(tab: &mut [T; N]) {
    for j in 0..N {
        let mut i_min = j;

        for i in j+1..N {
            if tab[i_min].partial_cmp(&tab[i]) == Some(Ordering::Greater) {
                i_min = i;
            }
        }
        tab.swap(j, i_min);
    }
}