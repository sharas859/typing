use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Counts {
    pub total: RwSignal<i32>,
    pub missed: RwSignal<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountsVec {
    pub data: Vec<(char, (i32, i32))>,
}

impl Counts {
    pub fn new(cx: Scope) -> Self {
        Self {
            total: create_rw_signal(cx, 0),
            missed: create_rw_signal(cx, 0),
        }
    }
}
