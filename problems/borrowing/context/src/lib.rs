#![forbid(unsafe_code)]

use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::rc::Rc;
use std::borrow::Borrow;

pub struct Context {
    kv_map: HashMap::<String, Rc::<dyn Any>>,
    singletone_map: HashMap::<TypeId, Rc::<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            kv_map: HashMap::new(),
            singletone_map: HashMap::new(),
        }
    }

    pub fn insert<T: 'static, K: Borrow<str>>(&mut self, key: K, obj: T) {
        let rc: Rc::<dyn Any> = Rc::new(obj);
        self.kv_map.insert(key.borrow().to_string(), rc); // old val will be dropped, ok.
    }

    pub fn get_rm<T: 'static, K: Borrow<str>>(&mut self, key: K) -> T {
        if let Some(rc) = self.kv_map.remove(key.borrow()) {
            if let Ok(rct) = rc.downcast::<T>() {
                if let Ok(obj) = Rc::try_unwrap(rct) {
                    return obj
                }
            }
        }
        panic!("No elem with such key")
    }

    pub fn get<T: 'static>(&self, key: &str) -> &T {
        if let Some(rc) = self.kv_map.get(key) {
            if let Some(objref) = (*rc).downcast_ref::<T>() {
                return objref
            }
        }
        panic!("No elem with such key")
    }

    pub fn insert_singletone<T: 'static>(&mut self, obj: T) {
        let rc: Rc::<dyn Any> = Rc::new(obj);
        self.singletone_map.insert(TypeId::of::<T>(), rc); // old val will be dropped, ok.
    }

    pub fn get_rm_singletone<T: 'static>(&mut self) -> T {
        if let Some(rc) = self.singletone_map.remove(&TypeId::of::<T>()) {
            if let Ok(rct) = rc.downcast::<T>() {
                if let Ok(obj) = Rc::try_unwrap(rct) {
                    return obj
                }
            }
        }
        panic!("No such singletone")
    }

    pub fn get_singletone<T: 'static>(&self) -> &T {
        if let Some(rc) = self.singletone_map.get(&TypeId::of::<T>()) {
            if let Some(objref) = (*rc).downcast_ref::<T>() {
                return objref
            }
        }
        panic!("No such singletone")
    }
}
