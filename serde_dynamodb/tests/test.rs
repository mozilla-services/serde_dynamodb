extern crate rusoto_dynamodb;
extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate serde_dynamodb;

use rusoto_dynamodb::AttributeValue;
use std::collections::{HashMap, HashSet};

#[test]
fn cant_serialize_non_struct() {
    let number: u8 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number: u16 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number: u32 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number: u64 = 5;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());

    let number: f32 = 5.1;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());
    let number: f64 = 5.2;
    assert!(serde_dynamodb::to_hashmap(&number).is_err());

    let none: Option<f64> = None;
    assert!(serde_dynamodb::to_hashmap(&none).is_err());

    let some: Option<f64> = Some(13.54);
    assert!(serde_dynamodb::to_hashmap(&some).is_err());
}

#[test]
fn can_serialize_struct() {
    #[derive(Serialize)]
    struct Basic {
        i: i32,
        f: f32,
    }
    let value = Basic { i: 5, f: 10.2 };
    assert!(serde_dynamodb::to_hashmap(&value).is_ok())
}

#[test]
fn can_deserialize_struct() {
    #[derive(Deserialize, Debug)]
    struct Internal {
        k: i32,
    }
    #[derive(Deserialize, Debug)]
    struct Basic {
        i: i32,
        j: i32,
        intern: Internal,
    }
    let mut value = HashMap::new();
    value.insert(
        "i".to_string(),
        AttributeValue {
            n: Some("5".to_string()),
            ..Default::default()
        },
    );
    value.insert(
        "j".to_string(),
        AttributeValue {
            n: Some("12".to_string()),
            ..Default::default()
        },
    );
    let mut intern = HashMap::new();
    intern.insert(
        "k".to_string(),
        AttributeValue {
            n: Some("27".to_string()),
            ..Default::default()
        },
    );
    value.insert(
        "intern".to_string(),
        AttributeValue {
            m: Some(intern),
            ..Default::default()
        },
    );
    let res: serde_dynamodb::error::Result<Basic> = serde_dynamodb::from_hashmap(value);
    assert!(res.is_ok());
}

#[test]
fn can_go_back_and_forth() {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Internal {
        k: i32,
        f: f64,
    }
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Basic {
        i: i32,
        j: i32,
        f: f32,
        d: f64,
        b: u8,
        u: u32,
        c: char,
        intern: Internal,
        list: Vec<i32>,
        some: Option<Internal>,
        none: Option<Internal>,
        complex: Vec<Option<Internal>>,
    }
    let value = Basic {
        i: 18,
        j: 74,
        f: 21.55,
        d: -45206.153,
        b: 13,
        u: 312,
        c: 0 as char,
        intern: Internal { k: 512, f: 13.54 },
        list: vec![0, 2, 5],
        some: Some(Internal { k: 120, f: 144.304 }),
        none: None,
        complex: vec![None, Some(Internal { k: 10, f: 12.56 })],
    };
    let hm = serde_dynamodb::to_hashmap(&value).unwrap();
    let out: Basic = serde_dynamodb::from_hashmap(hm).unwrap();
    assert_eq!(value, out);
}

#[test]
fn can_serialize_struct_leveled() {
    #[derive(Serialize)]
    struct Internal {
        i: i32,
    }
    #[derive(Serialize)]
    struct Basic {
        intern: Internal,
    }
    let value = Basic {
        intern: Internal { i: 5 },
    };
    assert!(serde_dynamodb::to_hashmap(&value).is_ok())
}

#[test]
fn can_deserialize_hashset() {
    #[derive(Deserialize, Debug, PartialEq)]
    struct Foo {
        bar: HashSet<String>,
        baz: HashSet<u32>,
    }
    let mut value: HashMap<String, AttributeValue> = HashMap::new();
    value.insert(
        "bar".to_string(),
        AttributeValue {
            ss: Some(vec!["foo".to_owned(), "bar".to_owned()]),
            ..Default::default()
        },
    );
    value.insert(
        "baz".to_string(),
        AttributeValue {
            ns: Some(vec!["3".to_owned(), "4".to_owned(), "5".to_owned()]),
            ..Default::default()
        },
    );

    let foo: Foo = serde_dynamodb::from_hashmap(value).unwrap();
    let mut expected = HashSet::new();
    expected.insert("foo".to_owned());
    expected.insert("bar".to_owned());
    assert_eq!(foo.bar, expected);

    let mut expected = HashSet::new();
    expected.insert(3);
    expected.insert(4);
    expected.insert(5);
    assert_eq!(foo.baz, expected);
}

/*#[test]
fn cant_serialize_array_of_non_struct() {
    assert!(serde_dynamodb::to_hashmap(&vec!(1, 2, 3)).is_err())
}
*/
