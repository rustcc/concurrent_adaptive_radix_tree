use super::{NodeHeader, NodeType, Node};

pub(crate) const EMPTY_MARKER: u8 = 48;

#[repr(C)]
#[repr(align(8))]
pub struct Node48 {
    header: NodeHeader,
    child_idx: [u8; 256],
    next_empty: u8,
    children: [*mut NodeHeader; 48],
}

impl AsRef<NodeHeader> for Node48 {
    fn as_ref(&self) -> &NodeHeader {
        &self.header
    }
}

impl AsRef<NodeType> for Node48 {
    fn as_ref(&self) -> &NodeType {
        &NodeType::N48
    }
}

impl Node48 {
    pub(crate) fn init_empty(&mut self) {
        for i in self.child_idx.iter_mut() {
            *i = EMPTY_MARKER;
        }
        self.next_empty = 0;
        for (i,child) in self.children.iter_mut().enumerate() {
            *child = (i + 1) as *mut NodeHeader;
        }
    }
}


impl<K: AsRef<[u8]> ,V> Node<K,V> for Node48 {
    fn is_full(&self) -> bool {
        self.header.count == 48
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let pos = self.next_empty;
        self.next_empty = self.children[pos as usize] as usize as u8;
        let key = key.as_ref()[0];
        self.child_idx[key as usize] = pos;
        self.children[pos as usize] = todo!("alloc node");
        self.header.count += 1;
        Some(todo!("return old value"))
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let key = key.as_ref()[0];
        let pos = self.child_idx[key as usize];
        let old = self.children[pos as usize];
        self.child_idx[key as usize] = EMPTY_MARKER;
        self.next_empty = pos;
        self.header.count -= 1;
        Some(todo!("return old value"))
    }

    fn change(&mut self, key: K, value: V) -> Option<V> {
        let old = self.children[self.child_idx[key.as_ref()[0] as usize] as usize];
        self.children[self.child_idx[key.as_ref()[0] as usize] as usize] = todo!("alloc node");
        Some(todo!("return old value"))
    }

    fn lookup(&self, key: K) -> Option<V> {
        if self.child_idx[key.as_ref()[0] as usize] == EMPTY_MARKER {
            None
        } else {
            let pos = self.child_idx[key.as_ref()[0] as usize];
            let ret = self.children[pos as usize];
            Some(todo!("return value"))
        }
    }
}