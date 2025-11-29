use crate::*;

#[test]
fn test_str_arrays() {
    let arr = ["abc", "one two three", "", "abc123", "foo"]
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    crate::test_serde(arr);
}
