use std::{sync::atomic::AtomicUsize, mem::ManuallyDrop};

use smallvec::SmallVec;

pub trait Node<K, V>
where
    K: AsRef<[u8]>,
    Self: AsRef<NodeHeader> + AsRef<NodeType>
{
    fn is_full(&self) -> bool;

    fn insert(&mut self, key: K, value: V) -> Option<V>;
    fn remove(&mut self, key: K) -> Option<V>;
    fn change(&mut self, key: K, value: V) -> Option<V>;

    fn lookup(&self, key: K) -> Option<V>;
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeType {
    N4 = 0x00,
    N16 = 0x01,
    N48 = 0x02,
    N256 = 0x03,
    // leaf
    N1 = 0xFF,
}

#[repr(C)]
#[repr(align(64))]
pub struct NodeHeader {
    // 2b type| ignore | 1b lock | 1b obsolete
    lock: AtomicUsize,
    count: usize,
    prefix: SmallVec<[u8; 8]>,
}

pub mod n16;
pub mod n256;
pub mod n4;
pub mod n48;
