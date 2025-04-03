#![forbid(unsafe_code)]

use std::str::Chars;

struct State<'a> {
    iters: Vec<Chars<'a>>,
}

impl<'a> State<'a> {
    pub fn new(strs: Vec<&'a str>) -> Self {
        let mut vec = Vec::<Chars<'a>>::new();
        for str in strs {
            vec.push(str.chars());
        }
        Self { iters: vec }
    }
    pub fn fill_next_start_symbols(&mut self, syms: &mut Vec<char>) {
        syms.clear();
        for it in &mut self.iters {
            if let Some(ch) = it.next() {
                syms.push(ch.clone());
            }
        }
    }
}

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    let mut syms = Vec::<char>::new();
    let mut res = String::new();
    let strs_amount = strs.len();
    let mut state = State::new(strs);
    loop {
        state.fill_next_start_symbols(&mut syms);
        if syms.len() != strs_amount {
            // one of strs ended
            return res;
        }
        let mut sym_it = syms.iter();
        match sym_it.next() {
            Some(ch) => {
                for ch_ in sym_it {
                    if ch_ != ch {
                        return res;
                    }
                }
                res.push(ch.clone());
            }
            None => return res,
        }
    }
}
