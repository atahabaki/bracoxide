use str_expand::expand;
fn main() {
    let input = "a{b,c{d,e}f,g}h";
    println!("RESULT: {:#?}", expand(input).unwrap());
}
