//! Util functions

use num_format::{Locale, ToFormattedString};

/// Format a number with thousands separator
pub fn fmt_nb(n: u32) -> String {
    n.to_formatted_string(&Locale::en)
}

/// Check if struct has the right traits
pub fn is_normal<T: Sized + Send + Sync + Unpin>() {}
