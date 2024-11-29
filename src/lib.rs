pub mod kmp;
pub mod raita;
pub mod simple;

#[cfg(test)]
mod test;

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

pub trait SliceReplace<T: PartialEq + Clone>: SliceFind<T> {
    fn replace(&self, old: impl AsRef<[T]>, new: impl AsRef<[T]>) -> Vec<T> {
        let mut this = self.as_ref().to_vec();
        let old = old.as_ref();
        let new = new.as_ref();

        if old == new {
            return this;
        }

        let old_len = old.len();
        let new_len = new.len();

        let mut part = &mut this[..];
        let mut maybe_pos = part.find(old);
        while let Some(pos) = maybe_pos {
            if new_len == old_len {
                part[pos .. pos+new_len].clone_from_slice(new);
                part = &mut part[pos+new_len ..];
            } else {
                let prefix = &part[..pos];
                let suffix = &part[pos+old_len ..];
                this = prefix.iter().chain(new.iter()).chain(suffix).map(|x| { x.to_owned() }).collect();
                part = &mut this[pos+new_len ..];
            }

            maybe_pos = part.find(old);
        }

        this
    }

}

impl<T: PartialEq+Clone> SliceReplace<T> for Vec<T> {}
impl<T: PartialEq+Clone> SliceReplace<T> for [T] {}
impl<T: PartialEq+Clone, const N: usize> SliceReplace<T> for [T; N] {}

