fn main() {
    let content = "A{B,C{D,E}F,G}H{J,K}L{3..5}";
    println!("Content: {}", content);
    if let Ok(tokens) = bracoxide::tokenizer::tokenize(content) {
        if let Ok(node) = bracoxide::parser::parse(&tokens) {
            println!("{:?}", bracoxide::expand(&node));
            if let Ok(possibilities) = bracoxide::expand(&node) {
                for possibility in possibilities {
                    println!("Possibility: {}", possibility);
                }
            }
        }
    }
}
