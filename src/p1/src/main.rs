use minicbor::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
struct Foo {
    #[n(0)]
    x: u32,

    #[n(1)]
    y: u32,
}

fn main() {
    let result = minicbor::to_vec(Foo { x: 1u32, y: 2u32 }).expect("Serialization error");
    let foo: Foo = minicbor::decode(&result).expect("Deserialization error");

    println!("{:?}", foo);
}