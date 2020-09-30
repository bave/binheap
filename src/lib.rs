
//#[derive(Debug)]
pub struct BinHeap<T: Ord>
{
    vec: Vec<T>,
    cmp: fn(&T, &T) -> bool
}   

impl<T: Ord> BinHeap<T>
{
    pub fn new() -> Self
    {
        BinHeap {
            vec: Vec::new(),
            cmp: |a, b| { 
                if a.cmp(b) == std::cmp::Ordering::Less {
                    true
                } else {
                    false
                }
            }
        }       
    }

    pub fn new_cmp(f: fn(&T, &T) -> bool) -> Self
    {
        BinHeap {
            vec: Vec::new(),
            cmp: f
        }       
    }

    pub fn with_capacity(cap: usize) -> Self
    {
        BinHeap {
            vec: Vec::with_capacity(cap),
            cmp: |a, b| { 
                if a.cmp(b) == std::cmp::Ordering::Less {
                    true
                } else {
                    false
                }
            }
        }       
    }

    pub fn with_capacity_cmp(cap: usize, f: fn(&T, &T) -> bool) -> Self
    {
        BinHeap {
            vec: Vec::with_capacity(cap),
            cmp: f
        }       
    }

    pub fn len(&self) -> usize
    {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool
    {
        self.vec.is_empty()
    }

    pub fn into_vec(self) -> Vec<T>
    {
        self.into()
    }

    pub fn push(&mut self, entry: T)
    {
        let len = self.len();
        self.vec.push(entry);
        self.heap_up2(len);
    }

    pub fn pop(&mut self) -> Option<T>
    {
        match self.len() {
            0 => {
                None
            },
            _ => {
                let len = self.len() - 1;
                self.vec.swap(0, len);
                let ret = self.vec.swap_remove(len);
                self.heap_down2();
                Some(ret)
            }
        }
    }

    pub fn peek(&self) -> Option<&T>
    {
        if self.vec.is_empty() {
            None
        } else {
            Some(&self.vec[0])
        }
    }

    /*
    //fn test(&mut self) -> std::cmp::Ordering
    fn test(&mut self)
    {
        print!("{:?}\n", (self.cmp)(&self.vec[0], &self.vec[1]))
    }
    */

    /*
    fn heap_up(&mut self, idx: usize)
    {
        let mut idx = idx;
        while idx > 0 {
            let parent = (idx - 1) >> 1;
            if &self.vec[idx] < &self.vec[parent] {
                self.vec.swap(parent, idx);
            }
            idx = parent;
        }
    }

    fn heap_down(&mut self)
    {
        let mut idx = 0;
        let end = self.len();
        let mut child_base = (idx << 1) + 1;
        while child_base < end {
            let left = child_base;
            let right = child_base + 1;
            let child = if right < end {
                if &self.vec[left] < &self.vec[right] {
                    left
                } else {
                    right
                }
            } else {
                left
            };
            if &self.vec[child] < &self.vec[idx] {
                self.vec.swap(child, idx);
                idx = child; 
                child_base = (idx << 1) + 1;
            } else {
                break;
            }
        }
    }
    */

    fn heap_up2(&mut self, idx: usize)
    {
        let mut idx = idx;
        while idx > 0 {
            let parent = (idx - 1) >> 1;
            //if self.vec[idx] < &self.vec[parent] {
            if (self.cmp)(&self.vec[idx], &self.vec[parent]) {
                self.vec.swap(parent, idx);
            }
            idx = parent;
        }
    }

    fn heap_down2(&mut self)
    {
        let mut idx = 0;
        let end = self.len();
        let mut child_base = (idx << 1) + 1;
        while child_base < end {
            let left = child_base;
            let right = child_base + 1;
            let child = if right < end {
                //if &self.vec[left] < &self.vec[right] {
                if (self.cmp)(&self.vec[left], &self.vec[right]) {
                    left
                } else {
                    right
                }
            } else {
                left
            };
            //if &self.vec[child] < &self.vec[idx] {
            if (self.cmp)(&self.vec[child], &self.vec[idx]) {
                self.vec.swap(child, idx);
                idx = child; 
                child_base = (idx << 1) + 1;
            } else {
                break;
            }
        }
    }
}

// -- trait impl --
// for Default
impl<T: Ord> Default for BinHeap<T>
{   
    fn default() -> Self
    {   
        BinHeap::new()
    }
}

// for self.into()
// std::convert::From<BinHeap<T>> to std::vec::Vec<T>
impl<T: Ord> From<BinHeap<T>> for Vec<T> {
    fn from(heap: BinHeap<T>) -> Vec<T> {
        heap.vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utils() -> Result<(), String> {
        let mut bh = BinHeap::new() as BinHeap<u64>;
        bh.push(1);
        bh.pop();
        assert_eq!(bh.len(), 0);
        assert_eq!(bh.is_empty(), true);
        //let v = bh.into_vec();
        //print!("{:?}\n", v);
        Ok(())
    }

    #[test]
    fn test_minheap() -> Result<(), String> {
        let mut bh = BinHeap::new() as BinHeap<u64>;
        bh.push(8);
        bh.push(7);
        bh.push(6);
        bh.push(5);
        bh.push(1);
        bh.push(2);
        bh.push(3);
        bh.push(4);
        assert_eq!(bh.pop(), Some(1));
        assert_eq!(bh.pop(), Some(2));
        assert_eq!(bh.pop(), Some(3));
        assert_eq!(bh.pop(), Some(4));
        assert_eq!(bh.pop(), Some(5));
        assert_eq!(bh.pop(), Some(6));
        assert_eq!(bh.pop(), Some(7));
        assert_eq!(bh.pop(), Some(8));
        assert_eq!(bh.pop(), None);
        Ok(())
    }

    #[test]
    fn test_maxheap() -> Result<(), String> {
        fn cmp(a: &u64, b: &u64) -> bool {
            if a.cmp(b) == std::cmp::Ordering::Greater {
                true
            } else {
                false
            }
        }
        let mut bh = BinHeap::new_cmp(cmp) as BinHeap<u64>;
        bh.push(8);
        bh.push(7);
        bh.push(6);
        bh.push(5);
        bh.push(1);
        bh.push(2);
        bh.push(3);
        bh.push(4);
        assert_eq!(bh.pop(), Some(8));
        assert_eq!(bh.pop(), Some(7));
        assert_eq!(bh.pop(), Some(6));
        assert_eq!(bh.pop(), Some(5));
        assert_eq!(bh.pop(), Some(4));
        assert_eq!(bh.pop(), Some(3));
        assert_eq!(bh.pop(), Some(2));
        assert_eq!(bh.pop(), Some(1));
        assert_eq!(bh.pop(), None);
        Ok(())
    }

    #[test]
    fn test_derive_struct() -> Result<(), String> {
        #[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
        struct MyEntry {
            a: u64
        }
        let mut bh = BinHeap::new() as BinHeap<MyEntry>;
        bh.push(MyEntry{a: 8});
        bh.push(MyEntry{a: 7});
        bh.push(MyEntry{a: 6});
        bh.push(MyEntry{a: 5});
        bh.push(MyEntry{a: 1});
        bh.push(MyEntry{a: 2});
        bh.push(MyEntry{a: 3});
        bh.push(MyEntry{a: 4});

        assert_eq!(bh.pop(), Some(MyEntry{a:1}));
        assert_eq!(bh.pop(), Some(MyEntry{a:2}));
        assert_eq!(bh.pop(), Some(MyEntry{a:3}));
        assert_eq!(bh.pop(), Some(MyEntry{a:4}));
        assert_eq!(bh.pop(), Some(MyEntry{a:5}));
        assert_eq!(bh.pop(), Some(MyEntry{a:6}));
        assert_eq!(bh.pop(), Some(MyEntry{a:7}));
        assert_eq!(bh.pop(), Some(MyEntry{a:8}));

        Ok(())
    }

    #[test]
    fn test_self_impl_struct() -> Result<(), String> {
        struct MyEntry {
            a: u64
        }
        impl std::fmt::Debug for MyEntry {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("MyEntry").field("a", &self.a).finish()
            }
        }
        impl Ord for MyEntry {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.a.cmp(&other.a)
            }
        }
        impl PartialOrd for MyEntry {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl PartialEq for MyEntry {
            fn eq(&self, other: &Self) -> bool {
                self.a == other.a
            }
        }
        impl Eq for MyEntry { }

        let mut bh = BinHeap::new() as BinHeap<MyEntry>;
        bh.push(MyEntry{a: 8});
        bh.push(MyEntry{a: 7});
        bh.push(MyEntry{a: 6});
        bh.push(MyEntry{a: 5});
        bh.push(MyEntry{a: 1});
        bh.push(MyEntry{a: 2});
        bh.push(MyEntry{a: 3});
        bh.push(MyEntry{a: 4});

        assert_eq!(bh.pop(), Some(MyEntry{a:1}));
        assert_eq!(bh.pop(), Some(MyEntry{a:2}));
        assert_eq!(bh.pop(), Some(MyEntry{a:3}));
        assert_eq!(bh.pop(), Some(MyEntry{a:4}));
        assert_eq!(bh.pop(), Some(MyEntry{a:5}));
        assert_eq!(bh.pop(), Some(MyEntry{a:6}));
        assert_eq!(bh.pop(), Some(MyEntry{a:7}));
        assert_eq!(bh.pop(), Some(MyEntry{a:8}));

        Ok(())
    }
}
