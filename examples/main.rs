#![recursion_limit = "500"]
#[macro_use(obj, val, arr)]
extern crate jst;
use jst::{Obj, Val};

// playground
fn main() {
    let key = "var_key";
    let var_value = "var_val";


    let person = obj! {
        name: "jhon",
        like_rust: true,
        like_go: null,
        emails : [
            "some@gmail.com",
            "some2@gmail.com"
        ],
        address: {
            city: "somewhere",
            zip: 5612
        },
        nested: {
            nested2:{
                key: "value",
                nested3: {
                    key: "value"
                }
            }
        },
        from_var: var_value,
        from_macro: val!("from_macro string"),
        from_value_var: Val::Str("from_value_var string".into()),
        empty_obj: {},
        empty_array: [],
        undefined_key: undefined,
        bool: true,
        "literal": true,
        [key]: "var_key",
        age: 56
    };

    // let arr = arr![45];

    // let array = arr![
    //     // "some@gmail.com",
    //     // 45,
    //     // ["value", 5],
    //     // {key: "value"},
    //     5,
    //     [],
    //     ...[23, "some val"],
    //     ...["sdfsf"],
    //     ...arr,
    //     {nested: {key: "value"}},
    //     "some value",
    //     // nested array
    //     [[["val"], "val"],[]],
    //     null,
    //     ...["some string"]
    // ];

    println!("{}", person);
    // println!("{}", val!(array));

}
