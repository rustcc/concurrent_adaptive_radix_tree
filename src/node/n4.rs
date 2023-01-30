use super::{NodeHeader, Node, NodeType};

#[repr(C)]
#[repr(align(64))]
pub struct Node4 {
    header: NodeHeader,
    keys: [u8; 4],
    children: [*mut NodeHeader; 4],
}

impl AsRef<NodeHeader> for Node4 {
    fn as_ref(&self) -> &NodeHeader {
        &self.header
    }
}

impl AsRef<NodeType> for Node4 {
    fn as_ref(&self) -> &NodeType {
        &NodeType::N4
    }
}

impl<K: AsRef<[u8]> ,V> Node<K,V> for Node4 {
    fn is_full(&self) -> bool {
        self.header.count == 4
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let mut pos: usize = 0;
        let mut key = key.as_ref();
        while pos < self.header.count {
            if self.keys[pos] < key[0] {
                pos += 1;
                continue;
            } else {
                break;
            }
        }
        unsafe {
            std::ptr::copy(
                self.keys.as_ptr().add(pos),
                self.keys.as_mut_ptr().add(pos + 1),
                self.header.count - pos,
            );
            std::ptr::copy(
                self.children.as_ptr().add(pos),
                self.children.as_mut_ptr().add(pos + 1),
                self.header.count - pos,
            );
        }
        self.keys[pos] = key[0];
        self.children[pos] = todo!("alloc node");
        self.header.count += 1;
    }

    fn remove(&mut self, key: K) -> Option<V> {
        for i in 0..self.header.count {
            if self.keys[i] == key.as_ref()[0] {
                let ret = self.children[i];
                unsafe {
                    std::ptr::copy(
                        self.keys.as_ptr().add(i + 1),
                        self.keys.as_mut_ptr().add(i),
                        self.header.count - i - 1,
                    );
                    std::ptr::copy(
                        self.children.as_ptr().add(i + 1),
                        self.children.as_mut_ptr().add(i),
                        self.header.count - i - 1,
                    );
                }
                self.header.count -= 1;
                return Some(todo!("{:?}", ret));
            }
        }
        None
    }

    fn change(&mut self, key: K, value: V) -> Option<V> {
        for i in 0..self.header.count {
            if self.keys[i] == key.as_ref()[0] {
                let ret = self.children[i];
                self.children[i] = todo!("alloc node");
                return Some(todo!("{:?}", ret));
            }
        }
        None
    }

    fn lookup(&self, key: K) -> Option<V> {
        for i in 0..self.header.count {
            if self.keys[i] == key.as_ref()[0] {
                return Some(todo!("{:?}", self.children[i]));
            }
        }
        None
    }
}
