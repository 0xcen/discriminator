fn main() -> () {
    let mut namespace = "global".to_string();
    let mut name = None;
    for arg in std::env::args().skip(1) {
        if arg == "-n" {
            namespace = std::env::args().nth(2).expect("no namespace given");
        } else {
            name = Some(arg);
        }
    }
    let name = name.expect("no name given");
    let hash = get_hash(&namespace, &name);

    // print result
    println!("namespace: {}", namespace);
    println!("name: {}", name);
    println!("hash: {:?}\n", hash);
    ()
}

pub fn get_hash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()[..8],
    );
    sighash
}
