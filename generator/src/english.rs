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
