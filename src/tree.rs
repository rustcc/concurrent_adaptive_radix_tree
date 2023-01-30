pub struct RawTree<K, V, A: Allocator + Clone + 'static = DefaultAllocator> {
    pub root: *mut Node256<V>,
    pub allocator: A,
    _phantom: PhantomData<K>,
}

impl<K, V, A: Allocator + Clone + 'static = DefaultAllocator> RawTree<K,V,A> {
    #[inline]
    fn insert_inner(
        k: &K
    ){
        let mut parent = None;
        let mut next = self.root as *const NodeHeader;
        let mut parent_key: u8;
        let mut node_key: u8 = 0;
        let mut level = 0;

        let mut node;

        loop {
            parent_key = node_key;
            node_key = k.as_bytes()[level];

            let next_node_tmp = node.as_ref().get_child(node_key);
            
        }
    }
}