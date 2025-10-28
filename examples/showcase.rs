use std::error::Error;

use bracoxide::explode;

fn main() -> Result<(), Box<dyn Error>> {
    let possibilities = explode("He then /eat {apple,banana,orange}...")?;
    for possibility in possibilities {
        println!("{possibility}");
    }
    Ok(())
}
