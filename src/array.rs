/// 長さを動的に指定でき、未初期化の要素を許容する配列
struct Array<T> {
    vec: Vec<MaybeUninit<T>>,
    len: usize,
}

impl<T> Array<T> {
    pub fn new(len: usize) -> Self {
        let mut vec = Vec::<MaybeUninit<T>>::with_capacity(len);
        vec.resize_with(len, MaybeUninit::uninit);
        Self { vec, len }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn replace(&mut self, idx: usize, new_value: MaybeUninit<T>) -> MaybeUninit<T> {
        std::mem::replace(&mut self.vec[idx], new_value)
    }

    pub fn move_contents_to(mut self, other: &mut Self) {
        other.vec.clear();
        other.vec.append(&mut self.vec);
        other.len = self.len;
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = MaybeUninit<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.vec[index]
    }
}

/// `start..end`から、start < x <= end の範囲を降順でイテレートするイテレータを作る
fn rev(range: Range<usize>) -> impl Iterator<Item = usize> {
    let r = (range.start + 1)..(range.end + 1);
    r.rev()
}

use std::{
    mem::MaybeUninit,
    ops::{Index, IndexMut, Range},
};

use crate::traits::List;

pub struct ArrayStack<T> {
    array: Array<T>,
    n: usize,
}

impl<T> ArrayStack<T> {
    pub fn with_initial_alloc(len: usize) -> Self {
        let array = Array::<T>::new(len);
        Self { array, n: 0 }
    }

    pub fn resize(&mut self) {
        todo!()
    }
}

impl<T> List<T> for ArrayStack<T> {
    fn size(&self) -> usize {
        self.n
    }

    fn get(&self, i: usize) -> Option<&T> {
        if i < self.array.len() {
            Some(unsafe { self.array[i].assume_init_ref() })
        } else {
            None
        }
    }

    fn set(&mut self, i: usize, v: T) -> Option<T> {
        if i < self.array.len() {
            let old = self.array.replace(i, MaybeUninit::new(v));
            Some(unsafe { old.assume_init() })
        } else {
            None
        }
    }

    fn add(&mut self, i: usize, v: T) {
        if self.n + 1 >= self.array.len() {
            self.resize();
        }

        let mut prev = self.array.replace(i, MaybeUninit::new(v));
        self.n += 1;

        for j in (i + 1)..(self.n + 1) {
            prev = self.array.replace(j, prev);
        }
    }

    fn remove(&mut self, i: usize) -> Option<T> {
        if i >= self.array.len() {
            return None;
        }

        let old = self.array.replace(i, MaybeUninit::uninit());

        let mut next = MaybeUninit::uninit();
        for j in (i..self.n).rev() {
            next = self.array.replace(j, next);
        }
        self.n -= 1;

        Some(unsafe { old.assume_init() })
    }
}
