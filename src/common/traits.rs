use crate::common::structs::{Counts, CountsVec};
use leptos::*;
use linked_hash_map::LinkedHashMap;
pub trait IncrCounts {
    fn incr_counts(&mut self, c: char, missed: bool);
}

impl IncrCounts for LinkedHashMap<char, Counts> {
    fn incr_counts(&mut self, c: char, missed: bool) {
        if let Some(entry) = self.get_mut(&c) {
            entry.total.update(|x| *x += 1);
            if missed {
                entry.missed.update(|x| *x += 1);
            }
        }
    }
}

pub trait Vectorize {
    fn from_map(map: LinkedHashMap<char, Counts>) -> Self;
    fn into_map(self, cx: Scope) -> LinkedHashMap<char, Counts>;
}
impl Vectorize for CountsVec {
    fn from_map(map: LinkedHashMap<char, Counts>) -> Self {
        let data = map
            .iter()
            .map(|(k, v)| (*k, (v.total.get_untracked(), v.missed.get_untracked())))
            .collect();
        CountsVec { data }
    }
    fn into_map(self, cx: Scope) -> LinkedHashMap<char, Counts> {
        let map: LinkedHashMap<char, Counts> = self
            .data
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    Counts {
                        total: create_rw_signal(cx, v.0),
                        missed: create_rw_signal(cx, v.1),
                    },
                )
            })
            .collect();
        map
    }
}
