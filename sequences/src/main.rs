

// logics copied from https://github.com/COMBINE-lab/kmers/
pub type Base = u8;
pub const A: Base = 0;
pub const C: Base = 1;
pub const G: Base = 2;
pub const T: Base = 3;


pub fn encode_binary(c: char) -> Base {
    // might have to play some tricks for lookup in a const
    // array at some point
    match c {
        'A' | 'a' => A,
        'C' | 'c' => C,
        'G' | 'g' => G,
        'T' | 't' => T,
        _ => panic!("cannot decode {c} into 2 bit encoding"),
    }
}



fn main() {
    
    let s = String::from("AGCTACGT");
    println!("How Rust can look at the sequence {s}");
    let t = s.as_bytes();
    println!("as_bytes {t:?}");
    let st = std::str::from_utf8(t).unwrap();
    println!("as str {st:?}");

    let mut w = 0_u8;
    for c in st.chars().rev() {
        w <<= 2;
        w |= encode_binary(c);
    }
    println!("and as single u8 integer {w}");
    println!("which looks like this in binary form: {w:b}");

    let faster =std::str::from_utf8( b"AGCTAGCT").unwrap();

    for c in faster.chars().rev() {
        w <<= 2;
        w |= encode_binary(c);
    }
    println!("and now in one line from string to binary: {w:b}");

}
