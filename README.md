# arson

**arson** is a simple rust json library for parsing string. it has nice formatted colored output

---

## Cargo.toml

```toml
[dependencies]
arson = "x.x"
```

## Example

```rust
use arson::{JSONError, Parser, JSON, JSON::*};

fn main() -> Result<(), JSONError> {
    // alternative A
    let json_str = std::fs::read_to_string("ex.json").unwrap();
    // alternative B
    let json_str = r#"{
        "name": "John Doe",
        "age": 43,
        "address": {
            "street": "10 Downing Street",
            "city": "London"
        },
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    } "#;

    // alternative 1
    let json = json_str.parse::<JSON>().expect("Failed to parse json");
    // alternative 2
    let json = Parser::parse(json_str.chars())?;

    println!("{:?}", json);

    match json {
        Array(val) => {}  // Vec<JSON>
        Object(val) => {} // HashMap<String, JSON>
        String(val) => {} // String
        Number(val) => {} // f64
        Bool(val) => {}   // bool
        Null => {}
    }

    Ok(())
}
```

Output

```
{
    "address": {
        "city": "London"[39m,
        "street": "10 Downing Street"[39m,
    },
    "name": "John Doe"[39m,
    "age": 43,
    "phones": [
        +44 1234567,
        +44 2345678,
    ],
}
```
