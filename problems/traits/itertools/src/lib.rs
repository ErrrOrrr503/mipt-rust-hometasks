#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;

pub struct LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    container: Vec<I::Item>,
    index: usize,
    iter_ended: bool,
    iter: I,
}

impl<I> LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn new(iter: I) -> Self
    {
        Self {
            container: Vec::new(),
            index: 0,
            iter_ended: false,
            iter: iter,
        }
    }
}

impl<I> Iterator for LazyCycle<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item>
    {
        let item: Option<Self::Item>;
        if !self.iter_ended {
            item = self.iter.next()
        }
        else {
            item = None;
        }
        match item {
            Some(item) => {
                self.container.push(item.clone());
                Some(item)
            }
            None => {
                self.iter_ended = true;
                if self.container.is_empty() {
                    None
                } else {
                    let retval = self.container[self.index].clone();
                    self.index = (self.index + 1) % self.container.len();
                    Some(retval)
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub struct Extract<I: Iterator> {
    container: VecDeque<I::Item>,
    iter: I,
}

impl<I: Iterator> Extract<I>
{
    pub fn new(mut iter: I, ext_ind: usize) -> (Option<I::Item>, Self)
    {
        let mut container = VecDeque::new();
        for _ in 0 .. ext_ind {
            match iter.next() {
                Some(item) => container.push_back(item),
                None => break,
            }
        }
        (iter.next(), Self {
            container: container,
            iter: iter,
        })
    }
}

impl<I: Iterator> Iterator for Extract<I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item>
    {
        if !self.container.is_empty() {
            self.container.pop_front()
        } else {
            self.iter.next()
        }
    }
}

////////////////////////////////////////////////////////////////////////////////


pub struct Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    diff_container: Rc<RefCell<VecDeque<I::Item>>>,
    master_first: Rc<RefCell<bool>>,
    is_master: bool,
    iter: Rc<RefCell<I>>,
    iter_exhausted: Rc<RefCell<bool>>,
}

impl<I> Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    pub fn tee(iter: I) -> (Tee<I>, Tee<I>)
    {
        let diff_container = Rc::new(RefCell::new(VecDeque::new()));
        let master_first = Rc::new(RefCell::new(false));
        let saved_iter = Rc::new(RefCell::new(iter));
        let iter_exhausted = Rc::new(RefCell::new(false));
        (Tee {
            diff_container: diff_container.clone(),
            master_first: master_first.clone(),
            is_master: true,
            iter: saved_iter.clone(),
            iter_exhausted: iter_exhausted.clone(),
        },
        Tee {
            diff_container: diff_container,
            master_first: master_first,
            is_master: false,
            iter: saved_iter,
            iter_exhausted: iter_exhausted,
        })
    }
}

impl<I> Iterator for Tee<I>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item>
    {
        match (self.is_master, self.master_first.borrow_mut()) {
            (is_master, master_first) if is_master == *master_first => {
                if *self.iter_exhausted.borrow() {
                    None
                } else {
                    let ret = self.iter.borrow_mut().next();
                    match ret {
                        Some(r) => {
                            self.diff_container.borrow_mut().push_back(r.clone());
                            Some(r)
                        },
                        None => {
                            *self.iter_exhausted.borrow_mut() = true;
                            None
                        }
                    }
                }
            },
            (is_master, mut master_first) if is_master != *master_first => {
                let ret = self.diff_container.borrow_mut().pop_front();
                if ret.is_some() {
                    ret
                } else {
                    let new_master_first = !*master_first;
                    *master_first = new_master_first;
                    if *self.iter_exhausted.borrow() {
                        None
                    } else {
                        let ret = self.iter.borrow_mut().next();
                        match ret {
                            Some(r) => {
                                self.diff_container.borrow_mut().push_back(r.clone());
                                Some(r)
                            },
                            None => {
                                *self.iter_exhausted.borrow_mut() = true;
                                None
                            }
                        }
                    }
                }
            },
            (_, _) => {
                unreachable!();
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////


pub struct GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    iter: I,
    func: F,
    first_group_elem: Option<I::Item>,
}

impl<I, F, V> GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    fn new(iter: I, f: F) -> GroupBy<I, F, V> {
        GroupBy {
            iter: iter,
            func: f,
            first_group_elem: None,
        }
    }
}

impl<I, F, V> Iterator for GroupBy<I, F, V>
where
    I: Iterator,
    F: FnMut(&I::Item) -> V,
    V: Eq,
{
    type Item = (V, Vec<I::Item>);

    fn next(&mut self) -> Option<(V, Vec<I::Item>)> {
        let item = if let Some(fge) = self.first_group_elem.take() {
            Some(fge)
        } else {
            self.iter.next()
        };
        if let Some(item) = item {
            let key = (self.func)(&item);
            let mut container = vec![item];
            while let Some(next_item) = self.iter.next() {
                if (self.func)(&next_item) == key {
                    container.push(next_item);
                } else {
                    self.first_group_elem = Some(next_item);
                    break;
                }
            }
            Some((key, container))
        } else {
            None
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait ExtendedIterator: Iterator {
    fn lazy_cycle(self) -> LazyCycle<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        LazyCycle::new(self)
    }


    fn extract(self, index: usize) -> (Option<Self::Item>, Extract<Self>)
    where
        Self: Sized,
    {
        Extract::new(self, index)
    }

    fn tee(self) -> (Tee<Self>, Tee<Self>)
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Tee::tee(self)
    }

    fn group_by<F, V>(self, func: F) -> GroupBy<Self, F, V>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> V,
        V: Eq,
    {
        GroupBy::new(self, func)
    }
}

impl<T: Iterator> ExtendedIterator for T {}