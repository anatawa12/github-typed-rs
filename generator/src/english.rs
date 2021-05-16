pub fn as_singular(str: &str) -> String {
    if let Some(no_suffix) = str.strip_suffix("ies") {
        format!("{}y", no_suffix)
    } else if let Some(no_suffix) = str.strip_suffix("fixes") {
        format!("{}fix", no_suffix)
    } else if let Some(no_suffix) = str.strip_suffix("statuses") {
        format!("{}status", no_suffix)
    } else if let Some(no_suffix) = str.strip_suffix("dispatches") {
        format!("{}dispatch", no_suffix)
    } else if let Some(no_suffix) = str.strip_suffix("s") {
        no_suffix.to_string()
    } else {
        str.to_string()
    }
}

pub fn is_plural(str: &str) -> bool {
    str.ends_with("s")
}

//noinspection SpellCheckingInspection
pub fn pascalize(str: &str) -> String {
    let mut r = String::with_capacity(str.len());
    for s in str.split(&[' ', '_', '-'] as &[char]).filter(|x| !x.is_empty()) {
        let bytes = s.as_bytes();
        let first = bytes[0];
        if "abcdefghijklmnopqrstuvwxyz".contains(first as char) {
            unsafe {
                let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes()[(first - 'a' as u8) as usize];
                r.push_str(std::str::from_utf8_unchecked(&[upper]));
                r.push_str(std::str::from_utf8_unchecked(&bytes[1..]));
            }
        } else {
            r.push_str(s);
        }
    }
    r
}
