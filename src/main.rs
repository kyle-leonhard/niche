// cargo +nightly rustc -- -Zprint-type-sizes

use std::num::{NonZero, NonZeroU8};

fn main() {
    print_memory_representation(Some('c'));
    print_memory_representation(Some(true));
    print_memory_representation(Some(false));
    let b: Option<bool> = None;
    print_memory_representation(b);
    print_memory_representation(Inner::A(2));
    print_memory_representation(Inner::B(3));
    print_memory_representation(Outer::C(5));
    print_memory_representation(Outer::D(Inner::B(7)));
    print_memory_representation(OuterOuter::F(Outer::D(Inner::B(9))));
    let mut v: Vec<u8> = Vec::with_capacity(8);
    v.push(1);
    print_memory_representation(v);

    let mut n: Option<NonZeroU8> = NonZero::new(5);
    print_memory_representation(n);
    n = None;
    print_memory_representation(n);

    let mut m: Option<u8> = Some(5);
    print_memory_representation(m);
    m = None;
    print_memory_representation(m);
}

#[derive(Debug)]
enum Inner {
    A(u32),
    B(u32),
}

#[derive(Debug)]
enum Outer {
    C(u32),
    D(Inner),
}

#[derive(Debug)]
enum OuterOuter {
    E(u32),
    F(Outer),
}

/// Print the memory representation of a value of a type T.
fn print_memory_representation<T: std::fmt::Debug>(t: T) {
    print!(
        "type={} value={t:?} size={}: ",
        std::any::type_name::<T>(),
        std::mem::size_of::<T>()
    );
    let start = &t as *const _ as *const u8;
    for i in 0..std::mem::size_of::<T>() {
        print!("{:02x} ", unsafe { *start.add(i) });
    }
    println!();
}
