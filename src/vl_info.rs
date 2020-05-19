use std::ops::Add::{self};

// use aa::bb as cc;
// kann auch ohne import mit aa::bb genutzt werden (wie bei Java)

// public  (pub) - von jedem Parant angesprochen werden
// private (std) - direkter Parent kann drauf zugreigen
// childs können immer drauf zugreifen
// Eigentlich wie in Java

fn main() { // argc: isize, argv: c_void
    println!("Hello world!");

    // for Schleife
    count_up_to(3);

    // while Schleife
    let t = get_input_until("end\n", false);
    println!("{}", t);

    // struct match enum
    struct_match_enum();

    // Generics, Traits, Implementations
    print!("{}\n\n", generic_sum(&[1, 2, 3, 4]));
    print_traits();

    // Closure
    let add_one = |x: u32| -> u32 {x + 1};
    let var :u32 = 3;
    print!("{}, {}\n", add_one(var), add_one(var + 2));

    // Threads
    start_threads();


}

fn start_threads() {

}

fn print_traits() {
    // umfassende Funktionen für Iteratoren (traits) in der Std.-Library
    // (map, zip, teake_while, skip_while, count, max, min, ...)
    // collect: wandelt Iterator Generic um; Generic muss FromIterator implementieren
    // let _v: Vec<i32> = vec![1,2,3].into_iter().map(|x| x * 2).collect();

    struct Attributes {
        s1: String,
        s2: String
    }
    trait T {
        fn f(&self);
    }

    impl T for Attributes {
        fn f(&self) {
            print!("{}\t{}\n\n", self.s1, self.s2);
        }
    }

    let obj = Attributes {
        s1: String::from("first str"),
        s2: String::from("second str")
    };
    obj.f();
}

fn generic_sum<T: Add<Output = T> + Copy>(list: &[T]) -> T {
    let mut sum: T = list[0];
    for &t_elm in list[1..].iter() {
        sum = sum + t_elm;
    }
    return sum;
}

fn struct_match_enum() {
    struct S {
        u: i32,
        v: i32,
    }
    enum E {
        Action1 {x: i32, y: i32},
        Action2 {x: i32, y: i32, z: i32},
        Action3 {},
    }

    use E::*;

    let u = 1;
    let s_imp = S {u, v: 2};

    let mut e_imp = E::Action3 {};
    match e_imp {
        Action1 {x, y} => {println!("{}, {}", x, y)},
        Action2 {x, y, z} => {println!("{}, {}, {}", x, y, z)},
        _ => {println!("Not Found!")},
    }

    e_imp = E::Action2 {x: s_imp.u, y: s_imp.v, z: 0};
    let z = match e_imp {
        E::Action1 {x, y} => {println!("{}, {}", x, y); -1},
        E::Action2 {x, y, z} => {println!("{}, {}", x, y); z},
        _ => {println!("Not Found!"); -1},
    };
    println!("Extracted z: {}.", z);

    e_imp = E::Action1 {x: s_imp.u, y: s_imp.v};
    match e_imp {
        E::Action1 {x, y} => {println!("{}, {}", x, y)},
        E::Action2 {x, y, z} => {println!("{}, {}, {}", x, y, z)},
        _ => {println!("Not Found!")},
    }
}

fn get_input_until(end: &str, print_steps: bool) -> String{
    let mut text: String = String::new(); // liest aktuelle Zeile
    let mut all: String = String::new(); // bildet den return String

    while &text != end { // Test, ob 'end' (und \n) eingegeben wurde
        all.push_str(&text); // letzte Zeile zu all hinzufügen
        text.clear(); // String leeren
        std::io::stdin().read_line(&mut text).expect("Zeile konnte nicht gelesen werden."); // Zeile lesen
        if print_steps {
            println!("{}", text);
        }
    }
    return all;
}

fn count_up_to(index: isize) {
    for i in 0..index {
        print!("{}\n", i);
    }
}