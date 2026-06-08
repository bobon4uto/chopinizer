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
