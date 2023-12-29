use num_format::{Locale, ToFormattedString};

pub fn fmt_nb(n: u32) -> String {
    n.to_formatted_string(&Locale::en)
}
