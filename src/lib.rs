pub mod kmp;
pub mod raita;
pub mod simple;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Algorithm {
    KMP,
    Raita,
    Simple,
}

impl Algorithm {
    pub fn slice_find<T: PartialEq + Ord>(&self, haystack: &[T], needle: &[T]) -> Option<usize> {
        match self {
            Self::KMP => kmp::slice_find(haystack, needle),
            Self::Raita => raita::slice_find(haystack, needle),
            Self::Simple => simple::slice_find(haystack, needle),
        }
    }
    pub fn slice_contains<T: PartialEq + Ord>(&self, haystack: &[T], needle: &[T]) -> bool {
        self.slice_find(haystack, needle).is_some()
    }
}

pub fn slice_find<T: PartialEq + Ord>(algo: Algorithm, haystack: &[T], needle: &[T]) -> Option<usize> {
    algo.slice_find(haystack, needle)
}
pub fn slice_contains<T: PartialEq + Ord>(algo: Algorithm, haystack: &[T], needle: &[T]) -> bool {
    algo.slice_contains(haystack, needle)
}

pub trait SliceFind<T: PartialEq>: AsRef<[T]> {
    fn find(&self, needle: impl AsRef<[T]>) -> Option<usize> {
        kmp::slice_find(self.as_ref(), needle.as_ref())
    }
    fn contains(&self, needle: impl AsRef<[T]>) -> bool {
        self.find(needle).is_some()
    }
}

impl<T: PartialEq> SliceFind<T> for Vec<T> {}
impl<T: PartialEq> SliceFind<T> for [T] {}
impl<T: PartialEq, const N: usize> SliceFind<T> for [T; N] {}

