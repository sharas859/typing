use instant::{Duration, Instant};
use leptos::*;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Counts {
    pub total: RwSignal<i32>,
    pub missed: RwSignal<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CountsVec {
    pub data: Vec<(String, (i32, i32))>,
}

impl Counts {
    pub fn new(cx: Scope) -> Self {
        Self {
            total: create_rw_signal(cx, 0),
            missed: create_rw_signal(cx, 0),
        }
    }
}

struct Character {
    pub symbol: char,
    pub missed: bool,
    pub time: Duration,
}

struct Lesson {
    pub characters: Vec<Character>,
    pub current: usize,
}

impl Lesson {
    pub fn new() -> Self {
        Self {
            characters: Vec::new(),
            current: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            characters: Vec::with_capacity(capacity),
            current: 0,
        }
    }

    pub fn update(&mut self, c: char, time: Instant) {
        if self.characters[self.current].symbol == c {
            self.characters[self.current].time = time.elapsed();
        } else {
            self.characters[self.current].missed = true;
        }
    }

    pub fn hit_rate(&self) -> f32 {
        let mut total = 0;
        let mut missed = 0;
        for c in &self.characters {
            if c.missed {
                missed += 1;
            }
            total += 1;
        }
        1.0 - (missed as f32 / total as f32)
    }

    pub fn wpm(&self) -> f32 {
        if self.characters.is_empty() {
            return 0.0;
        }
        let mut total = 0;
        let mut time = Duration::from_millis(0);
        for c in &self.characters {
            if !c.missed {
                total += 1;
                time += c.time;
            }
        }
        const MINUTE: f32 = 60.0;
        const LETTERS_PER_WORD: f32 = 5.0;
        let time = time.as_secs_f32() / MINUTE;
        total as f32 / LETTERS_PER_WORD / time
    }
}

impl fmt::Display for Lesson {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for c in &self.characters {
            write!(f, "{}", c.symbol)?;
        }
        Ok(())
    }
}

impl From<&str> for Lesson {
    fn from(s: &str) -> Self {
        let mut lesson = Self::with_capacity(s.len());
        for c in s.chars() {
            lesson.characters.push(Character {
                symbol: c,
                missed: false,
                time: Duration::from_millis(0),
            });
        }
        lesson
    }
}

//impl IntoView for Lesson {
//    fn into_view(self, cx: Scope) -> View {
//        view! {
//            cx,
//            {self.characters.into_iter().map(|c| {
//
//                view! {
//                    cx,
//                    <div>
//                        {c.symbol}
//                    </div>
//                }
//            }).collect_view(cx)}
//        }
//    }
//}
