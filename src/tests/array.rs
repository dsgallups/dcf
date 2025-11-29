fn str_array() -> Vec<String> {
    ["abc", "one two three", "", "abc123", "foo"]
        .into_iter()
        .map(ToString::to_string)
        .collect()
}

#[test]
fn test_str_arrays() {
    let arr = ["abc", "one two three", "", "abc123", "foo"]
        .into_iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();

    crate::test_serde(arr);
}

#[test]
fn test_nested_str_arrays() {
    let arr = vec![str_array(), str_array(), str_array(), str_array()];

    let varint = crate::to_dcf(&arr);
    let decode: Vec<Vec<String>> = crate::from_dcf(&varint).unwrap();

    println!("arr: {arr:?}");
    println!("decode: {decode:?}");
    crate::test_serde(arr);
}
