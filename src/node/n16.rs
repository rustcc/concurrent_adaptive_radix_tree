use std::{ops::Index, simd::{SimdPartialEq, ToBitMask}};
use std::simd::SimdPartialOrd;
use super::{NodeHeader, Node, NodeType};

#[repr(C)]
#[repr(align(8))]
pub struct Node16 {
    header: NodeHeader,
    keys: [u8; 16],
    children: [*mut NodeHeader; 16],
}

impl AsRef<NodeHeader> for Node16 {
    fn as_ref(&self) -> &NodeHeader {
        &self.header
    }
}

impl AsRef<NodeType> for Node16 {
    fn as_ref(&self) -> &NodeType {
        &NodeType::N16
    }
}

impl Node16 {
    #[inline]
    fn get_child_pos(&self, key: u8) -> Option<usize> {
        // using rust std simd
        let mut pos = std::simd::u8x16::splat(key);
        // find the element equal to key
        let mut mask = pos.simd_eq(std::simd::u8x16::from_slice(&self.keys));
        // get the position of the first true element
        let mut pos = mask.to_bitmask().trailing_zeros();
        if pos < 16 {
            Some(pos as usize)
        } else {
            None
        }
    }

    #[inline]
    fn get_insert_pos(&self, key: u8) -> usize {
        // using rust std simd
        let mut pos = std::simd::u8x16::splat(key);
        // find the element less than key
        let mut mask = pos.simd_lt(std::simd::u8x16::from_slice(&self.keys));
        // get the position of the first true element
        let mut pos = mask.to_bitmask().trailing_zeros();
        if pos < 16 {
            pos as usize
        } else {
            self.header.count
        }
    }
}

impl<K: AsRef<[u8]> ,V> Node<K,V> for Node16 {
    fn is_full(&self) -> bool {
        self.header.count == 16
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let pos = self.get_insert_pos(key.as_ref()[0]);
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
        self.keys[pos] = key.as_ref()[0];
        self.children[pos] = todo!("alloc node");
        self.header.count += 1;
        Some(todo!())
    }

    fn remove(&mut self, key: K) -> Option<V> {
        let pos = self.get_child_pos(key.as_ref()[0])?;
        unsafe {
            std::ptr::copy(
                self.keys.as_ptr().add(pos + 1),
                self.keys.as_mut_ptr().add(pos),
                self.header.count - pos - 1,
            );
            std::ptr::copy(
                self.children.as_ptr().add(pos + 1),
                self.children.as_mut_ptr().add(pos),
                self.header.count - pos - 1,
            );
        }
        self.header.count -= 1;
        Some(todo!())
    }

    fn change(&mut self, key: K, value: V) -> Option<V> {
        let pos = self.get_child_pos(key.as_ref()[0])?;
        let old = self.children[pos];
        self.children[pos] = todo!("alloc node");
        Some(todo!())
    }

    fn lookup(&self, key: K) -> Option<V> {
        let pos = self.get_child_pos(key.as_ref()[0])?;
        let child = self.children[pos];
        todo!()
    }
}