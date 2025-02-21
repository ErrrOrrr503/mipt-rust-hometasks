#![forbid(unsafe_code)]
use std::rc::Rc;

pub struct PRef<T> {
    pdata: Rc<T>,
}

impl<T> std::ops::Deref for PRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &(*self.pdata)
    }
}

impl<T> Clone for PRef<T> {
    fn clone(&self) -> Self {
        Self {
            pdata: self.pdata.clone(),
        }
    }
}

impl<T> PRef<T> {
    pub fn new(value: T) -> Self {
        Self {
            pdata: Rc::new(value),
        }
    }
}


////////////////////////////////////////////////////////////////////////////////

pub struct PStack<T> {
    lower: Option<Rc<PStack<T>>>,
    len: usize,
    data: Option<PRef<T>>,
}

impl<T> Default for PStack<T> {
    fn default() -> Self {
        Self {
            lower: None,
            len: 0,
            data: None,
        }
    }
}

impl<T> Clone for PStack<T> {
    fn clone(&self) -> Self {
        Self {
            lower: self.lower.clone(),
            len: self.len,
            data: self.data.clone(),
        }
    }
}

impl<T> PStack<T> {
    pub fn new() -> Self {
        Self {
            lower: None,
            len: 0,
            data: None,
        }
    }

    pub fn push(&self, value: T) -> Self {
        Self {
            lower: Some(Rc::new(self.clone())),
            len: self.len + 1,
            data: Some(PRef::new(value)),
        }
    }

    pub fn pop(&self) -> Option<(PRef<T>, Self)> {
        if let Some(data) = &self.data {
            if let Some(rclower) = &self.lower {
                return Some((
                    data.clone(),
                    Self {
                        lower: (*rclower).lower.clone(),
                        len: self.len.saturating_sub(1),
                        data: (*rclower).data.clone(),
                    }
                ))
            }
        }
        None
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = PRef<T>> {
        PStackIterator::new(Some(self.clone()))
    }

    pub fn get_ref_lower(&self) -> Option<&PStack<T>> {
        if let Some(rc) = &self.lower {
            return Some(rc.as_ref())
        }
        None
    }

}

pub struct PStackIterator<T> {
    cur: Option<PStack<T>>
}

impl<'a, T> PStackIterator<T> {
    fn new(pstack: Option<PStack<T>>) -> Self {
        Self {
            cur: pstack,
        }
    }
}

impl<T> Iterator for PStackIterator<T> {
    type Item = PRef<T>;

    fn next(&mut self) -> Option<PRef<T>> {
        let ret = match &self.cur {
            Some(ps) => if let Some((pr, _)) = ps.pop() {Some(pr.clone())} else {None},
            _ => None
        };
        self.cur = match &self.cur {
            Some(cur_ref_ps) => if let Some((_, lo)) = cur_ref_ps.pop() {Some(lo)} else {None},
            _ => None
        };
        return ret
    }
}

