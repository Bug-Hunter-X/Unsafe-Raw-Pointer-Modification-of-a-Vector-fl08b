fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 4; // Safe and idiomatic way to modify a vector element
    println!(" {:?}", v);
}