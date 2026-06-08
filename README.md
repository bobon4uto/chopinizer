# Chopinizer
Chop your str's horewer you like!
## Motive
Rust has split methods for strings, and it is strong, but it is limited.
And I wanted something unlimited, but what can It be? Custom function, obviously.
So this library does not implement special splitting - you do. It is just an interface to creating an iterator.
For example, i want to split by word "split" but keep it as a separate token - with split its not easy, but with this its actually very simple.
I must say that this approach is heavily inspired by [Tsoding's sv_chop_by_delim](https://github.com/tsoding/sv),
but instead of chopping delim, keeps it.
## Example
This example shows how to split by space and by `split` word, but get rid of the space.
```rust
use chopinizer::Choppable;

fn word_getter(source: &str) -> Option<&str> {
    let delims = vec![" ", "split"];
    for delim in delims {
        if source.starts_with(delim) {
            return Some(&source[..delim.len()]);
        }
    }
    return None;
}

fn main() {
    for token in "this willsplitthe sentance "
        .start_chopping(&word_getter)
        .filter(|s| *s != " ")
    {
        println!("{token}");
    }
}
```
