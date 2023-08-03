use crate::common::structs::{Counts, CountsVec};
use leptos::*;
use linked_hash_map::LinkedHashMap;

pub type CountsMap = LinkedHashMap<String, Counts>;
pub trait IncrCounts {
    fn incr_counts(&mut self, key: String, missed: bool);
}

impl IncrCounts for CountsMap {
    fn incr_counts(&mut self, key: String, missed: bool) {
        if let Some(entry) = self.get_mut(&key) {
            entry.total.update(|x| *x += 1);
            if missed {
                entry.missed.update(|x| *x += 1);
            }
        }
    }
}

pub trait Vectorize {
    fn from_map(map: CountsMap) -> Self;
    fn into_map(self, cx: Scope) -> CountsMap;
}
impl Vectorize for CountsVec {
    fn from_map(map: CountsMap) -> Self {
        let data = map
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    (v.total.get_untracked(), v.missed.get_untracked()),
                )
            })
            .collect();
        CountsVec { data }
    }
    fn into_map(self, cx: Scope) -> CountsMap {
        let map: CountsMap = self
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
