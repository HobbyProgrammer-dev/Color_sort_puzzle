// use std::collections::HashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct Game {
    bottles: Vec<Bottle>,
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Bottle {
    bott: Vec<u8>,
    height: u16,
}

#[derive(Clone, Copy)]
pub struct Move {
    start: usize,
    end: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GameState {
    botts: Vec<u64>,
}

impl Game {
    pub fn new(height: u16, b: Vec<Vec<u8>>) -> Self {
        let mut v = Vec::new();
        for x in b {
            v.push(Bottle::new(height, x));
        }
        Self { bottles: v }
    }
    pub fn is_move_valid(&self, mv: &Move) -> bool {
        if mv.start == mv.end {
            return false;
        }
        let st = self.bottles[mv.start].peek();
        let nd = self.bottles[mv.end].peek();
        if st.is_none() {
            return false;
        }
        if nd.is_none() {
            return true;
        }
        if self.bottles[mv.end].is_full() {
            return false;
        }
        if self.bottles[mv.start].is_all_same() && self.bottles[mv.end].is_empty() {
            return false;
        }
        st.unwrap() == nd.unwrap()
    }
    pub fn make_move(&mut self, mv: &Move) -> Result<(), InvalidMoveError> {
        let res = self.is_move_valid(mv);
        while self.is_move_valid(mv) {
            let val = self.bottles[mv.start].pop().unwrap();
            self.bottles[mv.end].push(val);
        }
        if res {
            Ok(())
        } else {
            Err(InvalidMoveError {})
        }
    }
    pub fn get_all_valid_moves(&self) -> Vec<Move> {
        let mut mvs = Vec::new();
        for st in 0..self.bottles.len() {
            for nd in 0..self.bottles.len() {
                let mv = Move { start: st, end: nd };
                if self.is_move_valid(&mv) {
                    mvs.push(mv);
                }
            }
        }
        mvs
    }
    pub fn is_win(&self) -> bool {
        for x in &self.bottles {
            if !x.is_empty() && !x.is_all_same() {
                return false;
            }
        }
        true
    }
}

impl Bottle {
    fn new(height: u16, b: Vec<u8>) -> Self {
        let mut v = Vec::new();
        for x in b {
            v.push(x);
        }
        Self { bott: v, height }
    }
    fn is_full(&self) -> bool {
        self.bott.len() == self.height.into()
    }
    fn peek(&self) -> Option<u8> {
        self.bott.last().copied()
    }
    fn push(&mut self, val: u8) {
        self.bott.push(val);
    }
    fn pop(&mut self) -> Option<u8> {
        self.bott.pop()
    }
    fn as_prim(&self) -> u64 {
        let mut val: u64 = 0;
        for x in &self.bott {
            val <<= 8;
            val |= *x as u64;
        }
        val
    }
    fn is_empty(&self) -> bool {
        self.bott.is_empty()
    }
    fn is_all_same(&self) -> bool {
        if !self.is_full() {
            return false;
        }
        for x in &self.bott {
            if *x != self.bott[0] {
                return false;
            }
        }
        true
    }
}

impl GameState {
    pub fn new(g: &Game) -> Self {
        let mut v = Vec::new();
        for x in &g.bottles {
            let x_as_prim = x.as_prim();
            v.push(x_as_prim);
        }
        v.sort();
        Self { botts: v }
    }
}

impl Move {
    pub fn str(&self) -> String {
        format!(
            "Move from bottle {} to bottle {}",
            self.start + 1,
            self.end + 1
        )
    }
}

pub struct InvalidMoveError {}
