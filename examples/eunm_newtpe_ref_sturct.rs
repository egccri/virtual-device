use lazy_static::lazy_static;

lazy_static! {
    pub static ref STATIC_A: A = A::new();
};

fn main() {
    let a = A::new();

    let b1 = B::new(&a);
    let b2 = B::new(&a);

    let one = Type::TypeOne(&a);
    let two = Type::TypeTwo(&a);

    let other_one = Other::TypeOne(&*STATIC_A);
    let other_two = Other::TypeOne(&*STATIC_A);

    println!("{:?}, {:?}", b1, b2);

    println!(
        "{:?}, {:?}, {:?}, {:?}",
        one,
        two,
        one.do_some(),
        two.do_some()
    );

    println!("{:?}, {:?}", other_one, other_two);
}

#[derive(Debug)]
pub struct A {}

#[derive(Debug)]
pub struct B<'a> {
    ref_a: &'a A,
}

#[derive(Debug)]
pub enum Type<'a> {
    TypeOne(&'a A),
    TypeTwo(&'a A),
}

#[derive(Debug)]
pub enum Other {
    TypeOne(&'static A),
    TypeTwo(&'static A),
}

impl A {
    pub fn new() -> Self {
        A {}
    }
}

impl<'a> B<'a> {
    pub fn new(ref_a: &'a A) -> Self {
        B { ref_a }
    }
}

impl<'a> Type<'a> {
    fn do_some(&self) -> &'a A {
        match *self {
            Type::TypeOne(a) => a,
            Type::TypeTwo(a) => a,
        }
    }
}
