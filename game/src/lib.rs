use std::{fmt::UpperHex, hash::Hash, rc::Rc};

pub mod read_csv;


#[derive(Clone, Copy)]
pub struct Move {
    start: usize,
    end: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct GameState<T: UnsignedInt> {
    botts: Vec<T>,
}

pub trait GameRules<T: UnsignedInt> {
    fn new(height: u8, b: &[Vec<u8>], color_size: u8) -> Self ;
    fn is_move_valid(&self, mv: &Move) -> bool ;
    fn make_move(&mut self, mv: &Move) -> Result<(), InvalidMoveError> ;
    fn get_all_valid_moves(&self) -> Vec<Move> ;
    fn is_win(&self) -> bool ;
    fn get_gamestate(&self) -> GameState<T>;
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

impl<T: UnsignedInt> GameRules<T> for GameMemEff<T> {
    fn new(height: u8, b: &[Vec<u8>], color_size: u8) -> Self {
        let mut v = Vec::new();
        for x in b {
            v.push(BottleMemEff::new(height, x, color_size));
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
        if ! self.is_move_valid(mv) {
            return Err(InvalidMoveError {  });
        }
        while self.is_move_valid(mv) {
            let val = self.bottles[mv.start].pop().unwrap();
            self.bottles[mv.end].push(val);
        }
        Ok(())
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
    fn get_gamestate(&self) -> GameState<T> {
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
    color_size: u8,
}

impl<T> BottleMemEff<T> where T: UnsignedInt {
    fn new(height: u8, b: &Vec<u8>, color_size: u8) -> Self {
        let mut bott = T::from(0);
        let mut shift_val = 0;
        for &x in b {
            bott = bott.or(T::from(x).lshift(shift_val));
            shift_val += color_size;
        }
        Self { bott, height, color_size }
    }
    fn len(&self) -> u8 {
        let mut a = self.bott;
        let mut c = 0;
        while a > (T::from(0)) {
            c += 1;
            a = a.rshift(self.color_size);
        }
        c
    }
    fn is_full(&self) -> bool {
        self.len() >= self.height
    }
    fn peek(&self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        let mask = self.get_mask();
        let l = self.len();
        let shift = (l - 1) * self.color_size;
        let val = self.bott.and(T::from(mask).lshift(shift)).rshift(shift);
        Some(val.to_u8())
    }

    fn get_mask(&self) -> u8 {
        if self.color_size == 8 {
            u8::MAX
        } else {
            (1 << self.color_size) - 1
        }
    }
    fn push(&mut self, val: u8) {
        let shift = self.len() * self.color_size;
        self.bott = self.bott.or(T::from(val).lshift(shift));
    }
    fn pop(&mut self) -> Option<u8> {
        if self.is_empty() {
            return None;
        }
        let mask = self.get_mask();
        let l = self.len();
        let shift = (l - 1) * self.color_size;
        let val = self.bott.and(T::from(mask).lshift(shift));
        self.bott = self.bott.xor(val);
        let val = val.rshift(shift);
        Some(val.to_u8())
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
        let mask = T::from(self.get_mask());
        let first = a.and(mask);
        let mut d ;
        while a > (T::from(0)) {
            d = a.and(mask);
            if d != first {
                return  false;
            }
            a = a.rshift(self.color_size);
        }
        true
    }
}

pub trait UnsignedInt: Copy + From<u8> + Ord + PartialEq + Eq + Hash + UpperHex {
    fn or(self, other: Self) -> Self ;
    fn xor(self, other: Self) -> Self ;
    fn and(self, other: Self) -> Self ;
    fn lshift(self, other: u8) -> Self ;
    fn rshift(self, other: u8) -> Self ;
    fn to_u8(self) -> u8 ;
}

macro_rules! impl_unsigned_int {
    ($($t:ty), *) => {
        $(
            impl UnsignedInt for $t {
                fn or(self, other: Self) -> Self {
                    self | other
                }
                fn xor(self, other: Self) -> Self {
                    self ^ other
                }
                fn and(self, other: Self) -> Self {
                    self & other
                }
                fn lshift(self, other: u8) -> Self {
                    self << other
                }
                fn rshift(self, other: u8) -> Self {
                    self >> other
                }
                fn to_u8(self) -> u8 {
                    self as u8
                }
            }
        )*
    };
}

impl_unsigned_int!(u8, u16, u32, u64, u128);

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
                Some(prev) => current = Rc::clone(prev),
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

impl Default for Path {
    fn default() -> Self {
        Self::new()
    }
}
