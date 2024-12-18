use crate::*;

/// https://en.wikipedia.org/wiki/Raita_algorithm
pub fn slice_find<T: Ord>(haystack: &[T], needle: &[T]) -> Option<usize> {
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
    /*
    for item in needle.iter() {
        if ! haystack.contains(item) {
            return None;
        }
    }*/

    // Preprocess the bad match table
    let mut bm_bc = BTreeMap::new();

    // Update shifts for all but the last character
    for (i, item) in needle.iter().enumerate().take(needle_len - 1) {
        bm_bc.insert(item, needle_len - i - 1);
    }

    // Prematch-window values
    let first_item = &needle[0];
    let middle_item = &needle[needle_len / 2];
    let last_item = &needle[needle_len - 1];

    let mut pos = 0;
    while pos <= (haystack_len - needle_len) {
        let item = &haystack[pos+needle_len-1];
        if item == last_item
            && &haystack[pos] == first_item
            && &haystack[pos + needle_len/2] == middle_item
            && &haystack[pos+1 .. pos+needle_len-1] == &needle[1 .. needle_len-1]
        {
            return Some(pos);
        }

        // Use BTreeMap to get the shift value
        //pos += bm_bc.get(item).copied().unwrap_or(needle_len);
        pos += bm_bc.get(item).copied().unwrap_or(1);
    }

    None
}

pub fn slice_contains<T: Ord>(haystack: &[T], needle: &[T]) -> bool {
    slice_find(haystack, needle).is_some()
}

