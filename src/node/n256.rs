use super::{NodeHeader, NodeType, Node};

#[repr(C)]
#[repr(align(8))]
pub struct Node256 {
    header: NodeHeader,
    key_mask: [u8;32],
    children: [*mut NodeHeader; 256],
}

impl Node256 {
    #[inline]
    fn set_mask(&mut self, key: usize) {
        let idx = key / 8;
        let bit = key % 8;
        self.key_mask[idx] |= 1 << bit;
    }

    #[inline]
    fn unset_mask(&mut self, key: usize) {
        let idx = key / 8;
        let bit = key % 8;
        self.key_mask[idx] &= !(1 << bit);
    }

    #[inline]
    fn get_mask(&self, key: usize) -> bool {
        let idx = key / 8;
        let bit = key % 8;
        let key_mask = self.key_mask[idx];
        key_mask & (1 << bit) != 0
    }
}

impl AsRef<NodeHeader> for Node256 {
    fn as_ref(&self) -> &NodeHeader {
        &self.header
    }
}

impl AsRef<NodeType> for Node256 {
    fn as_ref(&self) -> &NodeType {
        &NodeType::N256
    }
}

impl<K: AsRef<[u8]> ,V> Node<K,V> for Node256 {
    fn is_full(&self) -> bool {
        self.header.count == 256
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.children[key.as_ref()[0] as usize] = todo!("alloc node");
        self.set_mask(key.as_ref()[0] as usize);
        self.header.count += 1;
    }

    fn remove(&mut self, key: K) -> Option<V> {
        self.unset_mask(key.as_ref()[0] as usize);
        let old = self.children[key.as_ref()[0] as usize];
        self.header.count -= 1;
        Some(todo!("return old value"))
    }

    fn change(&mut self, key: K, value: V) -> Option<V> {
        let old = self.children[key.as_ref()[0] as usize];
        self.children[key.as_ref()[0] as usize] = todo!("alloc node");
        Some(todo!("return old value"))
    }

    fn lookup(&self, key: K) -> Option<V> {
        if self.get_mask(key.as_ref()[0] as usize) {
            let child = self.children[key.as_ref()[0] as usize];
            Some(todo!("return value"))
        } else {
            None
        }
    }
}