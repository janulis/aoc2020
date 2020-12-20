use std::io::{self, BufRead};

pub trait VecItem {
    fn parse(&mut self, s: &str);
}

pub fn stdin_to_vec<T: VecItem + Default>() -> Vec<T> {
    let mut list: Vec<T> = vec![];

    for line_res in io::stdin().lock().lines() {
        let line = line_res.unwrap();

        let mut item: T = Default::default();
        item.parse(line.as_str());

        list.push(item);
    }

    list
}
