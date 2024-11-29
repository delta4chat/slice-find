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
    /*
    for item in needle.iter() {
        if ! haystack.contains(item) {
            return None;
        }
    }*/

    let mut pos = 0;
    while pos+needle_len <= haystack_len {
        if haystack[pos] == needle[0] {
            if &haystack[pos .. pos+needle_len] == needle {
                return Some(pos);
            }
        }

        pos += 1;
    }

    None
}

pub fn slice_contains<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    slice_find(haystack, needle).is_some()
}

