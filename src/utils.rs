fn id_to_uuid(path: &str) -> String {
    format!(
        "{x}-{y}-{z}-{a}-{b}",
        x = &path[..8],
        y = &path[8..12],
        z = &path[12..16],
        a = &path[16..20],
        b = &path[20..]
    )
}

pub fn parse_page_id(page_id: String) -> String {
    let id = page_id.replace("-", "");
    let path = slice_from_end(&id, 32);
    match path {
        Some(path) => id_to_uuid(path),
        None => page_id.to_string(), //really i should throw an error here
    }
    // id_to_uuid(path)
}

fn slice_from_end(s: &str, n: usize) -> Option<&str> {
    s.char_indices().rev().nth(n - 1).map(|(i, _)| &s[i..])
}
