#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;
    // Check special cases first
    if _first_list.is_empty() && _second_list.is_empty() {
        // If both are empty, they're equal
        return Equal;
    } else if _first_list.is_empty() {
        // If only the first is empty, of course it's a sublist of the other
        return Sublist;
    } else if _second_list.is_empty() {
        // Same if only the second one is empty
        return Superlist;
    // Done with special cases. Now check if sublist/superlist
    } else if _first_list.len() < _second_list.len() {
        // If first is smaller, it might be a sublist
        if check_sublist(_first_list, _second_list) {
            return Sublist;
        } else {
            return Unequal;
        }
    } else if _second_list.len() < _first_list.len() {
        // If first list is larger, it might be a superlist
        if check_sublist(_second_list, _first_list) {
            return Superlist;
        } else {
            return Unequal;
        }
    } else {
        // Remaining possibility, they're equal in length. Might be equal too
        if are_eq(_first_list, _second_list) {
            return Equal;
        } else {
            return Unequal;
        }
    }
}

fn check_sublist<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    for (i, val) in b.iter().enumerate() {
        // Search through the other list
        if val == a.get(0).unwrap() {
            // If there's a a place that starts with the first item of the
            // other list, it's a candidate for being equal
            if i + a.len() > b.len() {
                // Check that the index makes sense first
                break;
            }
            //  Compare the smaller slice with a section of the bigger slice
            if are_eq(a, &b[i..i + a.len()]) {
                return true;
            }
        }
    }
    false
}

fn are_eq<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    for (i, val) in b.iter().enumerate() {
        // Search through both arrays
        if a.get(i).unwrap() != val {
            // If anything that does not equal is found
            return false; // Immediately break out of the function
        }
    }
    true // Otherwise they seem to be equal!
}
