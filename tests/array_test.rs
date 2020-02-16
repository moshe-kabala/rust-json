#[macro_use(json, value, array)]
extern crate json;

use json::{Json, Value};

#[test]
fn test_array_macro() {
    let array = array![
        "some@gmail.com",
        45,
        ["value", 5],
        {key: "value"},
        5,
        [],
        // todo fix the bug in nested json
        // {nested: {key: "value"}},
        "some value"
    ];

    let empty_array = array![];

    assert_eq!(
        array,
        vec![
            value!("some@gmail.com"),
            value!(45),
            vec![value!("value").into(), 5.into()].into(),
            value!({key: "value"}),
            5.into(),
            vec![].into(),
            "some value".into()
        ]
    );
    let expected_empty_array: Vec<Value> = vec![];
    assert_eq!(empty_array, expected_empty_array);
}