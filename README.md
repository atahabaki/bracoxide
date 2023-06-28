# {Bracoxide}

[![Tests](https://github.com/atahabaki/bracoxide/actions/workflows/rust.yml/badge.svg)](https://github.com/atahabaki/bracoxide/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![Documentation](https://docs.rs/bracoxide/badge.svg)](https://docs.rs/bracoxide)
[![Bracoxide Crate](https://img.shields.io/crates/v/bracoxide.svg)](https://crates.io/bracoxide)

Bracoxide is a powerful Rust library for handling and expanding brace expansions.
It provides a simple and intuitive way to generate combinations and permutations
from brace patterns.

## Features

* __Brace Expansion__: Easily expand brace patterns into a list of all possible combinations.
* __Error Handling__: Comprehensive error handling for invalid brace patterns or expansion failures.
* __MIT Licensed__: Bracoxide is open-source and licensed under the MIT License.

## Installation

Add Bracoxide to your Cargo.toml:

```toml
[dependencies]
bracoxide = "0.1.0"
```

## Usage

Import the bracoxide crate and start expanding brace patterns:

```rust
use bracoxide::{bracoxidize, OxidizationError};

fn main() {
    let content = "foo{1..3}bar";
    match bracoxidize(content) {
        Ok(expanded) => {
            println!("Expanded patterns: {:?}", expanded);
        }
        Err(error) => {
            eprintln!("Error occurred: {:?}", error);
        }
    }
}
```

For more details and advanced usage, please refer to the [API documentation](https://docs.rs/bracoxide).

## Contributing

Contributions are welcome! If you encounter any issues or have ideas for improvements, 
please open an issue or submit a pull request. See our 
[contribution guidelines](Contributing.md) for more information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
