//! # Chopinizer
//! Chop your str's horewer you like!
//! ## Motive
//! Rust has split methods for strings, and it is strong, but it is limited.
//! And I wanted something unlimited, but what can It be? Custom function, obviously.
//! So this library does not implement special splitting - you do. It is just an interface to creating an iterator.
//! For example, i want to split by word "split" but keep it as a separate token - with split its not easy, but with this its actually very simple.
//! I must say that this approach is heavily inspired by [Tsoding's sv_chop_by_delim](https://github.com/tsoding/sv),
//! but instead of chopping delim, keeps it.
//! ## Example
//! This example shows how to split by space and by `split` word, but get rid of the space.
//! ```rust
//! use chopinizer::Choppable;
//!
//! fn word_getter(source: &str) -> Option<&str> {
//!     let delims = vec![" ", "split"];
//!     for delim in delims {
//!         if source.starts_with(delim) {
//!             return Some(&source[..delim.len()]);
//!         }
//!     }
//!     return None;
//! }
//!
//! fn main() {
//!     for token in "this willsplitthe sentance "
//!         .start_chopping(&word_getter)
//!         .filter(|s| *s != " ")
//!     {
//!         println!("{token}");
//!     }
//! }
//! ```

/// This trait is the only thing you need from this library, but If you like ChoppingBoard
/// interface more, feel free to use it.
pub trait Choppable {
/// turns string into iterator of tokens
    fn start_chopping<'a>(&'a mut self,word_getter:&'a dyn Fn(&str)->Option<&str>) -> ChoppingBoard<'a>;
/// returns token that sits at the start
/// word_getter should return the token. If None was returned, peek will gobbling everything until
/// Some is returned, or string ended, then it will return. returns None only if str is exhsausted.
    fn peek(&self, word_getter:&dyn Fn(&str)->Option<&str>) -> Option<&str>;
/// returns token that sits at the start, and chops it off
    fn chop(&mut self, word_getter:&dyn Fn(&str)->Option<&str>) -> Option<&str>;
}

impl Choppable for &str {
    fn start_chopping<'a>(&'a mut self,word_getter:&'a dyn Fn(&str)->Option<&str>) -> ChoppingBoard<'a> {
       ChoppingBoard::new(self, word_getter)
    }
    fn peek(&self, word_getter:&dyn Fn(&str)->Option<&str>) -> Option<&str> {
        let mut working_slice : &str = self;
        let mut unknown_return_len = 0;
        while working_slice.len() > 0 {
          let word = word_getter(&working_slice);
          match word {
            Some(word) => {
              if unknown_return_len == 0 {
                  return Some(word);
              } else {
                  return Some(&self[..unknown_return_len]);
              }
            },
            None => {
                working_slice = &working_slice[1..];
                unknown_return_len += 1;
            },
          }
        }

        if self.is_empty()  {
            None
        } else {
            Some(self)
        }
    }
    fn chop(&mut self, word_getter:&dyn Fn(&str)->Option<&str>) -> Option<&str> {
        let Some(word) = self.peek(word_getter) else { return None };
        // rust makes trivial stuff so complicated sometimes bru
        let len = word.len();
        let return_word = &self[..len];
        *self = &self[len..];
        Some(return_word)
    }
}

/// Only needed to create `Iterator`. Use `Choppable` since it has `start_chopping` that internally uses
/// this struct.
pub struct ChoppingBoard<'a> {
    source: &'a str,
    word_getter: &'a dyn Fn(&str)->Option<&str>,
}

impl<'a> ChoppingBoard<'a> {
    pub fn new(source:&'a str,word_getter:&'a dyn Fn(&str)->Option<&str>) -> Self {
        Self { source, word_getter }
    }
}

impl<'a> Iterator for ChoppingBoard<'a> {
    type Item = &'a str;
    fn next(&mut self) -> Option<&'a str> {
        let Some(word) = self.source.peek(self.word_getter) else { return None };
        // rust makes trivial stuff so complicated sometimes bru
        let len = word.len();
        let return_word : &str = &self.source[..len];
        self.source = &self.source[len..];
        Some(return_word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn basic_word_getter (source:&str) -> Option<&str> {
        if source.starts_with(" ") {
            return Some(&source[..1]);
        }
        return None;
    }
    #[test]
    fn peek_basic() {
        assert_eq!("123 456".peek(&basic_word_getter), Some("123"));
        assert_eq!(   " 456".peek(&basic_word_getter), Some(" "));
        assert_eq!(    "456".peek(&basic_word_getter), Some("456"));
        assert_eq!(       "".peek(&basic_word_getter), None);
    }
    #[test]
    fn chop_basic() {
        let mut source = "123 456";
        assert_eq!(source.chop(&basic_word_getter), Some("123"));
        assert_eq!(source.chop(&basic_word_getter), Some(" "));
        assert_eq!(source.chop(&basic_word_getter), Some("456"));
        assert_eq!(source.chop(&basic_word_getter), None);
    }
    #[test]
    fn chopping_board_basic() {
        let mut chopping_board = ChoppingBoard::new("123 456",&basic_word_getter);
        assert_eq!(chopping_board.next(), Some("123"));
        assert_eq!(chopping_board.next(), Some(" "));
        assert_eq!(chopping_board.next(), Some("456"));
        assert_eq!(chopping_board.next(), None);
    }
    #[test]
    fn chopping_board_basic_iter() {
        for (i, chopped) in "123 456".start_chopping(&basic_word_getter).enumerate() {
            if i == 0 {
                assert_eq!(chopped, "123");
            } else if i == 1 {
                assert_eq!(chopped, " ");
            } else if i == 2 {
                assert_eq!(chopped, "456");
            } else if i == 3 {
                unreachable!();
            }
        }
    }

}
