//! Common CLI abstractions shared across uv command modules.

pub mod child;
pub mod exit_status;
pub mod printer;

/// Format a list of items with "and" conjunction.
/// E.g., "foo", "bar", and "baz"
pub fn conjunction(names: Vec<String>) -> String {
    let mut names = names.into_iter();
    let first = names.next();
    let last = names.next_back();
    match (first, last) {
        (Some(first), Some(last)) => {
            let mut result = first;
            let mut comma = false;
            for name in names {
                result.push_str(", ");
                result.push_str(&name);
                comma = true;
            }
            if comma {
                result.push_str(", and ");
            } else {
                result.push_str(" and ");
            }
            result.push_str(&last);
            result
        }
        (Some(first), None) => first,
        _ => String::new(),
    }
}
