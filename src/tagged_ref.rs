use std::alloc::{alloc, Layout};
use std::mem::{size_of, transmute};

const ALIGNMENT: usize = 4;
const TAG_MASK: usize = 0b11;

#[derive(Clone, Copy, Debug)]
pub struct TaggedRef {
    ptr: usize,
}

impl TaggedRef {
    pub fn new<T>(value: T, tag: usize) -> Self {
        if tag > TAG_MASK {
            panic!("too large tag");
        }

        let layout = Layout::from_size_align(size_of::<T>(), ALIGNMENT).expect("valid layout");
        let ptr = unsafe { alloc(layout) };

        *unsafe { transmute::<*const u8, &mut T>(ptr) } = value;

        Self {
            ptr: ptr as usize | tag,
        }
    }

    pub fn tag(&self) -> usize {
        self.ptr & TAG_MASK
    }

    pub fn as_ref<T>(&self) -> &T {
        unsafe { &*((self.ptr & !TAG_MASK) as *const T) }
    }
}

#[cfg(test)]
mod test {
    use super::TaggedRef;

    #[test]
    fn new() {
        TaggedRef::new(42, 0);
    }

    #[test]
    fn tag() {
        assert_eq!(TaggedRef::new(42, 0).tag(), 0);
        assert_eq!(TaggedRef::new(42, 1).tag(), 1);
        assert_eq!(TaggedRef::new(42, 2).tag(), 2);
        assert_eq!(TaggedRef::new(42, 3).tag(), 3);
    }

    #[test]
    fn as_ptr() {
        assert_eq!(*TaggedRef::new(42 as usize, 3).as_ref::<usize>(), 42);
    }
}
