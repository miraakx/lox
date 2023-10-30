use std::ops::Index;

use crate::{value::Value, alias::IdentifierSymbol};

#[derive(Clone, Debug)]
pub enum TinyVec<T: Clone, const SIZE: usize>
{
    Stack([Option<T>; SIZE], usize),
    Heap(Vec<T>)
}

impl<const SIZE: usize> Default for TinyVec<Value, SIZE>
{
    fn default() -> Self {
        Self::new()
    }
}

impl <const SIZE: usize> TinyVec<(IdentifierSymbol, Value), SIZE>
{
    pub const fn new() -> Self
    {
        const INIT: Option<(IdentifierSymbol, Value)> = None;
        let arr: [Option<(IdentifierSymbol, Value)>; SIZE] = [INIT; SIZE];
        Self::Stack(arr, 0)
    }
}

impl <const SIZE: usize> TinyVec<Value, SIZE>
{
    pub const fn new() -> Self
    {
        const INIT: Option<Value> = None;
        let arr: [Option<Value>; SIZE] = [INIT; SIZE];
        Self::Stack(arr, 0)
    }
}

impl <T: Clone, const SIZE: usize> TinyVec<T, SIZE>
{
    pub fn push(&mut self, element: T)
    {
        match self
        {
            Self::Stack(arr, len) =>
            {
                if *len ==  SIZE {
                    let mut vec: Vec<T> = arr.iter().filter_map(|item| item.clone()).collect();
                    vec.push(element);
                    *self = Self::Heap(vec);
                } else {
                    arr[*len] = Some(element);
                    *len += 1;
                }
            },
            Self::Heap(vec) =>
            {
                vec.push(element);
            },
        }
    }

    pub fn set(&mut self, index: usize, element: T) {
        match self
        {
            Self::Stack(arr, len) =>
            {
                if index > *len {
                    panic!("insertion index (is {}) should be <= len (is {})", index, len);
                }
                arr[index] = Some(element);
            },
            Self::Heap(vec) =>
            {
                vec[index] = element;
            },
        }
    }

    pub fn len(&self) -> usize {
        match self
        {
            Self::Stack(_, len) =>
            {
                *len
            },
            Self::Heap(vec) =>
            {
                vec.len()
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct TinyVecIter<'a, T: Clone, const SIZE: usize>
{
    inner: &'a TinyVec<T, SIZE>,
    index: usize,
}

impl<'a, T: Clone, const SIZE: usize> IntoIterator for &'a TinyVec<T, SIZE>
{
    type Item     = &'a T;
    type IntoIter = TinyVecIter<'a, T, SIZE>;

    fn into_iter(self) -> Self::IntoIter
    {
        TinyVecIter {
            inner: self,
            index: 0,
        }
    }

}

impl<'a, T: Clone, const SIZE: usize> Iterator for TinyVecIter<'a, T, SIZE>
{
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item>
    {
        let current_value = match self.inner {
            TinyVec::Stack(arr, _) => {
                if self.index >= SIZE {
                    None
                } else {
                    arr[self.index].as_ref()
                }
            },
            TinyVec::Heap(vec) => {
                Some(&vec[self.index])
            },
        };
        self.index += 1;
        current_value
    }
}

impl<T: Clone, const SIZE: usize> Index<usize> for TinyVec<T, SIZE> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match &self {
            Self::Stack(array, len) => {
                array[index]
                    .as_ref()
                    .map_or_else(
                        || panic!("get index (is {}) should be <= len (is {})", index, len),
                        |value| value
                    )
            }
            Self::Heap(vec) => {
                &vec[index]
            }
        }
    }
}

#[test]
fn test_stack()
{
    let mut prova: TinyVec<i32, 3> = TinyVec::Stack([None, None, None], 0);
    prova.push(1);
    prova.push(2);
    prova.push(3);
    assert_eq!(prova[0], 1);
    assert_eq!(prova[1], 2);
    assert_eq!(prova[2], 3);
}

#[test]
fn test_heap()
{
    let mut prova: TinyVec<i32, 3> = TinyVec::Stack([None, None, None], 0);
    prova.push(1);
    prova.push(2);
    prova.push(3);
    prova.push(4);
    prova.push(5);
    assert_eq!(prova[0], 1);
    assert_eq!(prova[1], 2);
    assert_eq!(prova[2], 3);
    assert_eq!(prova[3], 4);
    assert_eq!(prova[4], 5);
}

#[test]
fn test_set()
{
    let mut prova: TinyVec<i32, 3> = TinyVec::Stack([None, None, None], 0);
    prova.push(1);
    prova.push(2);
    prova.push(3);
    prova.push(4);
    prova.push(5);
    prova.set(0, -1);
    prova.set(1, -2);
    prova.set(2, -3);
    prova.set(3, -4);
    prova.set(4, -5);
    assert_eq!(prova[0], -1);
    assert_eq!(prova[1], -2);
    assert_eq!(prova[2], -3);
    assert_eq!(prova[3], -4);
    assert_eq!(prova[4], -5);
}

#[test]
fn test_set_2()
{
    let mut prova: TinyVec<i32, 1> = TinyVec::Stack([None], 0);
    prova.push(1);
    prova.push(2);
    prova.push(3);
    prova.push(4);
    prova.push(5);
    prova.set(0, -1);
    prova.set(4, -2);
    prova.set(0, -3);
    prova.set(0, -4);
    prova.set(0, -5);
    assert_eq!(prova[0], -5);
    assert_eq!(prova[1], 2);
    assert_eq!(prova[2], 3);
    assert_eq!(prova[3], 4);
    assert_eq!(prova[4], -2);
}