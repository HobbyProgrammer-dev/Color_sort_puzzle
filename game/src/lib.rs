use std::rc::Rc;

pub mod read_csv;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Game {
    bottles: Vec<Bottle>,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct Bottle {
    bott: Vec<u8>,
    height: u8,
}

#[derive(Clone, Copy)]
pub struct Move {
    start: usize,
    end: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GameState<T: UnsignedInt> {
    botts: Vec<T>,
}

pub trait GameRules {
    fn new(height: u8, b: &Vec<Vec<u8>>) -> Self ;
    fn is_move_valid(&self, mv: &Move) -> bool ;
    fn make_move(&mut self, mv: &Move) -> Result<(), InvalidMoveError> ;
    fn get_all_valid_moves(&self) -> Vec<Move> ;
    fn is_win(&self) -> bool ;
    fn get_gamestate<T: UnsignedInt>(&self) -> GameState<T>;
}

impl GameRules for Game {
    fn new(height: u8, b: &Vec<Vec<u8>>) -> Self {
        let mut v = Vec::new();
        for x in b {
            v.push(Bottle::new(height, x));
        }
        Self { bottles: v }
    }
    fn is_move_valid(&self, mv: &Move) -> bool {
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
    fn make_move(&mut self, mv: &Move) -> Result<(), InvalidMoveError> {
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
    fn get_all_valid_moves(&self) -> Vec<Move> {
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
    fn is_win(&self) -> bool {
        for x in &self.bottles {
            if !x.is_empty() && !x.is_all_same() {
                return false;
            }
        }
        true
    }
    fn get_gamestate<T: UnsignedInt>(&self) -> GameState<T> {
        let mut v = Vec::new();
        for x in &self.bottles {
            let x_as_prim = x.as_prim();
            v.push(x_as_prim);
        }
        v.sort();
        GameState { botts: v }
    }
}

impl Bottle {
    fn new(height: u8, b: &Vec<u8>) -> Self {
        let mut v = Vec::new();
        for &x in b {
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
    fn as_prim<T: UnsignedInt>(&self) -> T {
        let mut val = T::from(0);
        for &x in &self.bott {
            val = val.lshift(8);
            val = val.or(T::from(x));
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

impl Move {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn str(&self) -> String {
        format!(
            "Move from bottle {} to bottle {}",
            self.start + 1,
            self.end + 1
        )
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GameMemEff<T: UnsignedInt> {
    bottles: Vec<BottleMemEff<T>>,
}

impl<T: UnsignedInt> GameRules for GameMemEff<T> {
    fn new(height: u8, b: &Vec<Vec<u8>>) -> Self {
        let mut v = Vec::new();
        for x in b {
            v.push(BottleMemEff::new(height, x));
        }
        Self { bottles: v }
    }
    fn is_move_valid(&self, mv: &Move) -> bool {
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
    fn make_move(&mut self, mv: &Move) -> Result<(), InvalidMoveError> {
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
    fn get_all_valid_moves(&self) -> Vec<Move> {
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
    fn is_win(&self) -> bool {
        for x in &self.bottles {
            if !x.is_empty() && !x.is_all_same() {
                return false;
            }
        }
        true
    }
    fn get_gamestate<U: UnsignedInt + Into<T>>(&self) -> GameState<U> {
        let mut v = Vec::new();
        for x in &self.bottles {
            let x_as_prim = x.as_prim();
            v.push(x_as_prim);
        }
        v.sort();
        GameState { botts: v }
    }
}

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
struct BottleMemEff<T: UnsignedInt> {
    bott: T,
    height: u8,
}

impl<T> BottleMemEff<T> where T: UnsignedInt {
    fn new(height: u8, b: &Vec<u8>) -> Self {
        let mut bott = T::from(0);
        let mut shift_val = 0;
        for &x in b {
            bott = bott.or(T::from(x).lshift(shift_val));
            shift_val += 8;
        }
        Self { bott, height }
    }
    fn len(&self) -> u8 {
        let mut a = self.bott;
        let mut c = 0;
        while a > (T::from(0)) {
            c = c + 1;
            a = a.rshift(8);
        }
        c
    }
    fn is_full(&self) -> bool {
        self.len() == self.height
    }
    fn peek(&self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        let mut a = self.bott;
        let mut d = 0;
        while a > (T::from(0)) {
            d = a.modulo(8);
            a = a.rshift(8);
        }
        Some(d as u8)
    }
    fn push(&mut self, val: u8) {
        let shift = self.len() * 8;
        self.bott = self.bott.or(T::from(val).lshift(shift));
    }
    fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        let mut a = self.bott;
        let mut d = 0;
        let mut shift = 0;
        while a > (T::from(0)) {
            d = a.modulo(8);
            shift += 8;
            a = a.rshift(8);
        }
        self.bott = self.bott.xor(T::from(d).lshift(shift * 8));
        Some(d)
    }
    fn as_prim(&self) -> T {
        self.bott
    }
    fn is_empty(&self) -> bool {
        self.bott == T::from(0)
    }
    fn is_all_same(&self) -> bool {
        if !self.is_full() {
            return false;
        }
        let mut a = self.bott;
        let first = a.modulo(8);
        let mut d ;
        while a > (T::from(0)) {
            d = a.modulo(8);
            if d != first {
                return  false;
            }
            a = a.rshift(8);
        }
        true
    }
}

trait UnsignedInt: Copy + From<u8> + Ord + PartialEq + Eq {
    fn or(self, other: Self) -> Self ;
    fn xor(self, other: Self) -> Self ;
    fn lshift(self, other: u8) -> Self ;
    fn rshift(self, other: u8) -> Self ;
    fn modulo(self, other: u8) -> u8 ;
}

impl UnsignedInt for u128 {
}

pub struct InvalidMoveError {}

pub struct Path {
    mv: Option<Move>,
    previous: Option<Rc<Path>>,
    depth: usize,
}

impl Path {
    pub fn new() -> Self {
        Self { mv: None, previous: None, depth: 0 }
    }
    pub fn add(self: &Rc<Self>, mv: Move) -> Self {
        let depth = self.depth + 1;
        Self { mv: Some(mv), previous: Some(Rc::clone(self)), depth }
    }
    pub fn print(self: &Rc<Path>) {
        let mut v = Vec::new();
        let mut current = Rc::clone(self);
        loop {
            if let Some(mv) = current.mv {
                v.push(mv);
            }
            match &current.previous {
                Some(prev) => current = Rc::clone(&prev),
                None => break
            }
        }
        while let Some(mv) = v.pop() {
            println!("{}", mv.str());
        }
    }
    pub fn get_depth(&self) -> usize {
        self.depth
    }
}

