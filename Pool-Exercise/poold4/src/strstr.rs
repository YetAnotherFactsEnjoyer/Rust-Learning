pub fn strstr<'a>(haystack: &'a str, needle: &'a str) -> &'a str {
    let v: Vec<_> = haystack.match_indices(needle).collect();

    if v.is_empty() {
        ""
    } else {
        let i = v[0].0;
        let (_, last) = haystack.split_at(i);
        last
    }
}
