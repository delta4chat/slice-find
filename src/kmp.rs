use crate::*;

// thanks to https://docs.rs/kmp
pub fn slice_find<T: PartialEq>(haystack: &[T], needle: &[T]) -> Option<usize> {
    let haystack_len = haystack.len();
    let needle_len = needle.len();

    // Quick edge cases
    if needle_len == 0 {
        return Some(0); // Empty pattern matches at the start
    }
    if needle_len > haystack_len {
        return None; // Pattern longer than text cannot match
    }

    if needle_len == 1 {
        // Special case for single-element pattern
        return haystack.iter().position(|c| { c == &needle[0] });
    }
    if needle_len == haystack_len {
        // Special case for length equal
        if haystack == needle {
            return Some(0);
        } else {
            return None;
        }
    }

    // The pre-check is a preliminary operation that determines the existence of each item in the needle within the haystack. It is agnostic to the order or contiguity of the items, as if one of needle's item is absent in haystack, it is impossible to match. Consequently, the pre-check saves computational resources by returning an early result.
    /*for item in needle.iter() {
        if ! haystack.contains(item) {
            return None;
        }
    }*/

    let mut lps = Vec::with_capacity(needle_len);
    lps.push(0);
    for item in needle.iter() {
        let mut j = lps.last().copied().unwrap();
        while j > 0 && item != &needle[j] {
            j = lps[j - 1];
        }
        if item == &needle[j] {
            j += 1;
        }
        lps.push(j);
    }

    let mut needle_pos = 0;
    for (haystack_pos, haystack_item) in haystack.iter().enumerate() {
        while needle_pos > 0 && haystack_item != &needle[needle_pos] {
            // item mismatch, move backwards in the needle
            needle_pos = lps[needle_pos - 1];
        }

        if haystack_item == &needle[needle_pos] {
            // char matches, move to next needle item
            needle_pos += 1;

            if needle_pos == needle_len {
                // we found all needle characters in the haystack, return position in haystack
                return Some(haystack_pos - (needle_pos - 1));
            }
        }
    }

    // not found
    None
}

pub fn slice_contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    slice_find(haystack, needle).is_some()
}

