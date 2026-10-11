// stage 0 courte introduction du lanage ou bien commencer

fn main() {
    println!("Présentation bases de rust :");
    println!("Variables ----");
    println!("i32 -> entier, f32 -> flotant, i64 et f64 -> entier 64 bits et flotant 64 bits");
    println!("bool -> boolean, char -> caractère Unicode");
    // les u32 , u64 ect .. sont des entier positifs uniquement, isize et usize sont de taille non définis mais prennent plus d'espace mémoire
    let mon_age: i32 = 25;
    let nb_flotant: f32 = 3.14;
    let lettre: char = 'A';
    println!("affiche une variable -> ");
    println!("Mon Age : {}", mon_age);
    println!("Pi : {}", nb_flotant);

    println!("Par défaut les variables sont immuables (non modifiables");
    let mut x: i32 = 45;
    x = x + mon_age + 42;
    println!("X = {}", x);
    println!("autre forme d'écriture pour addition: (x += y)");
    x += 45;
    println!("X + 45 = {}", x);

    // &str -> réference non modifiable, longueur fixe, string -> chaine à taille variable qui à ces propres données
    println!("Les String ");
    let nom: &str = "Damien";
    let mut message: String = String::from("Bonjour chouchou");
    // ajout un char , push_str() pour push les string
    message.push( '!');
    println!("{}", message);
    // {:?} -> quand le résultats peut être null
    // .chars().nth(n) -> permet de récupèrer un caractère à une position donnée
    println!("{:?}", message.chars().nth(2) );

    println!("Les Owner (propriétaire)");
    // on ne peut faire de transformation sur texte si on fait copir = texte car copie devient propriéatire de texte
    // Donc on utilise .clone() pour enregistrer une copie
    let mut texte = String::from("Rust");
    let copie = texte.clone();

    println!("Constante");
    const MAX: usize = 100;
    println!("constante max -> {}", MAX);

    println!("Les Array ---");
    // Tableau fixe on définis sa taille
    let nombres: [i32; 5] = [10, 20, 30, 40, 50];
    println!("Va afficher le premier numéro de la liste {}", nombres[0]);


    println!("Les boucles et conditions ---");
    // les if else
    let age = 20;

    if age >= 18 {
        println!("Majeur");
    } else {
        println!("Mineur");
    }

    // boucle for
    for i in 0..5 {
        println!("{}", i);
    }

    // boucle while
    let mut i = 0;
    while i < 5 {
        println!("{}", i);
        i += 1;
    }

    // boucle infinis on stop avec break
    loop {
        println!("Boucle");
        break;
    }

    // Les fonctions
    println!("résultat de la fonction addition ->");
    let resultat = addition(10, 20);
    println!("{}", resultat);

    afficher(&message);

    // Classe et objets en rust
    let mut counter = Counter::new();

    counter.increment();
    counter.increment();

    println!("{}", counter.get_value()); 

}

// fonction qui retourne un résultat
fn addition(a: i32, b: i32) -> i32 {
    return a + b;
}

// fonction qui retourne rien
fn afficher(message: &str) {
    println!("{}", message);
}


// Les classes struct et impl
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get_value(&self) -> i32 {
        self.value
    }
}

