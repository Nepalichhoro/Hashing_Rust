mod sha3_hash;

fn main() {
    let input = "hello world";

    let sha3_result = sha3_hash::compute_sha3(input);
    println!("SHA3-256 hash of '{}': {}", input, sha3_result);

}
