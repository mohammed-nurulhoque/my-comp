//! A namescope Module. A namescope is a series of nested scopes each having its bound
//! names. A scope can capture a name from a parent scope.
//! # FUTURE
//! Special case the static namescope because it shouldn't have Local and Capture,
//! and the inner scopes shouldn't have StaticFn or Capture
use std::collections::HashMap;
use crate::{
    types::Type,
    imper_ast::ValPath,
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;
    #[test]
    fn test() {
        let mut ns0 = NameScope { head: Some(Box::new(ScopeList {
            local: HashMap::from_iter(vec![
                ("foxbar".to_string(), (ValPath::Constructor, Type::Bool)),
                ("cnnbar".to_string(), (ValPath::Local(vec![0]), Type::Bool)),
            ]),
            captures_sz: 0,
            parent: NameScope { head: None },
        }))};
        let mut ns1 = NameScope { head: Some(Box::new(ScopeList {
            local: HashMap::from_iter(vec![
                ("bar".to_string(), (ValPath::Local(vec![0]), Type::Bool)),
                ("foobar".to_string(), (ValPath::CaptureLocal(0, vec![0]), Type::Bool)),
            ]),
            captures_sz: 1,
            parent: ns0,
        }))};
        let mut ns = NameScope { head: Some(Box::new(ScopeList {
            local: HashMap::from_iter(vec![("foo".to_string(), (ValPath::Local(vec![0]), Type::Bool))]),
            captures_sz: 0,
            parent: ns1,
        }))};
        assert_eq!(ns.get("foo").unwrap(), &(ValPath::Local(vec![0]), Type::Bool));
        assert_eq!(ns.head.as_ref().unwrap().captures_sz, 0);
        assert_eq!(ns.get("bar").unwrap(), &(ValPath::CaptureLocal(0, vec![0]), Type::Bool));
        assert_eq!(ns.head.as_ref().unwrap().captures_sz, 1);
        // add test cases
    }
}

pub struct NameScope {
    head: Option<Box<ScopeList>>,
}

struct ScopeList {
    pub local: HashMap<String, (ValPath, Type)>,
    captures_sz: u16,
    parent: NameScope,
}

struct IterMut<'a> {
    next: Option<&'a mut ScopeList>,
}

struct Iter<'a> {
    next: Option<&'a ScopeList>,
}

impl NameScope {
    pub fn new() -> Self {
        NameScope { head: Some(Box::new(ScopeList { 
            local: HashMap::new(), 
            captures_sz: 0, 
            parent: NameScope { head: None } 
        }))}
    }

    pub fn local(&mut self) -> &mut HashMap<String, (ValPath, Type)> {
        &mut self.head.as_mut().unwrap().local
    }

    fn iter(&self) -> Iter {
        Iter { next: self.head.as_ref().map(|node| &**node) }
    }

    fn iter_mut(&mut self) -> IterMut {
        IterMut { next: self.head.as_mut().map(|node| &mut **node) }
    }

    pub fn push_layer(&mut self) {
        let node = Box::new(ScopeList {
            local: HashMap::new(),
            captures_sz: 0,
            parent: NameScope { head: self.head.take() }
        });
        self.head = Some(node);
    }

    pub fn pop_layer(&mut self) -> HashMap<String, (ValPath, Type)> {
        self.head.take().map(|node| {
            *self = node.parent;
            node.local
        }).unwrap()
    }

    pub fn drain_local(&mut self) {
        self.head.as_mut().unwrap().local.retain(|_, v| if let (ValPath::Local(_), _) = v { false } else { true })
    }

    pub fn extend_local(&mut self, map: HashMap<String, (ValPath, Type)>) {
        self.head.as_mut().unwrap().local.extend(map)
    }

    pub fn exists(&self, key: &str) -> bool {
        for (ns, _) in self.iter() {
            if ns.get(key).is_some() {
                return true
            }
        }
        false
    }

    /// Get a name from a namescope, doing all captures as necessary
    /// # FUTURE
    /// after nll conditional control flow remove unsafe
    pub fn get(&mut self, key: &str) -> Option<&(ValPath, Type)> {
        if let Some(val) = self.head.as_mut().unwrap().local.get(key) {
            return unsafe {
                Some(&*(val as *const _))
            }
        }
        let mut lengths = vec![];       
        let mut result = None;
        for (map, captures_sz) in self.iter().skip(1) {
            match map.get(key) {
                None => (),
                Some(val @ (ValPath::StaticVal(_), _)) | Some(val @ (ValPath::Constructor, _)) => 
                    return unsafe { Some(&*(val as *const _)) },
                Some(val) => {
                    result = Some(val);
                    break;
                },
            }
            lengths.push(captures_sz);
        }
        if result.is_none() {
            return None
        }
        let (path, t) = result.unwrap().clone();
        let mut namescopes = self.iter_mut();
        for len in lengths {
            let (map, captures_sz) = namescopes.next().unwrap();
            insert_captured(map, captures_sz, key.to_string(), &ValPath::CaptureCaptured(len, 0), t.clone());
        }
        let (map, captures_sz) = namescopes.next().unwrap();
        insert_captured(map, captures_sz, key.to_string(), &path, t.clone());
        self.get(key)
    }
}


/// inserts a captured value in self.local given its path in parent and update capture_sz
fn insert_captured(
    map: &mut HashMap<String, (ValPath, Type)>, 
    captures_sz: &mut u16,
    key: String, path_up: &ValPath, t: Type
) -> Option<(ValPath, Type)> {
    let path_down = match path_up {
        &ValPath::Local(ref v) => ValPath::CaptureLocal(*captures_sz, v.clone()),
        &ValPath::CaptureLocal(u, _) | &ValPath::CaptureCaptured(u, _) => 
            ValPath::CaptureCaptured(*captures_sz, u),
        &ValPath::StaticVal(_) | &ValPath::Constructor => panic!("Capture static not expected"),
        
    };
    *captures_sz += 1;
    map.insert(key, (path_down, t))
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a HashMap<String, (ValPath, Type)>, u16);

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.parent.head.as_ref().map(|node| &**node);
            (&node.local, node.captures_sz)
        })
    }
}

impl<'a> Iterator for IterMut<'a> {
    type Item =  (&'a mut HashMap<String, (ValPath, Type)>, &'a mut u16);

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.parent.head.as_mut().map(|node| &mut **node);
            (&mut node.local, &mut node.captures_sz)
        })
    }
}