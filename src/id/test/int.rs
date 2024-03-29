use crate::Id;
use std::collections::HashMap;

struct Foo {}

#[test]
fn test_i64_partial_eq() {
    let id1: Id<Foo, i64> = Id::new(1);
    let id2: Id<Foo, i64> = Id::new(1);
    let id3: Id<Foo, i64> = Id::new(2);

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_i32_partial_eq() {
    let id1: Id<Foo, i32> = Id::new(1);
    let id2: Id<Foo, i32> = Id::new(1);
    let id3: Id<Foo, i32> = Id::new(2);

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_i64_inner() {
    let id1: Id<Foo, i64> = Id::new(1);
    assert_eq!(id1.inner(), &1);
}

#[test]
fn test_i32_inner() {
    let id1: Id<Foo, i32> = Id::new(1);
    assert_eq!(id1.inner(), &1);
}

#[test]
fn test_i32_clone() {
    let id1: Id<Foo, i32> = Id::new(1);
    assert_eq!(id1.clone(), id1);
}

#[test]
fn test_hash_key_i64() {
    let id1: Id<Foo, i64> = Id::new(1);
    let mut hash: HashMap<Id<Foo, i64>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[test]
fn test_hash_key_u64() {
    let id1: Id<Foo, u64> = Id::new(1);
    let mut hash: HashMap<Id<Foo, u64>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[test]
fn test_hash_key_i32() {
    let id1: Id<Foo, i32> = Id::new(1);
    let mut hash: HashMap<Id<Foo, i32>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[test]
fn test_hash_key_u32() {
    let id1: Id<Foo, u32> = Id::new(1);
    let mut hash: HashMap<Id<Foo, u32>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[test]
fn test_hash_key_i16() {
    let id1: Id<Foo, i16> = Id::new(1);
    let mut hash: HashMap<Id<Foo, i16>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[test]
fn test_hash_key_u16() {
    let id1: Id<Foo, u16> = Id::new(1);
    let mut hash: HashMap<Id<Foo, u16>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[test]
fn test_hash_key_i8() {
    let id1: Id<Foo, i8> = Id::new(1);
    let mut hash: HashMap<Id<Foo, i8>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[test]
fn test_hash_key_u8() {
    let id1: Id<Foo, u8> = Id::new(1);
    let mut hash: HashMap<Id<Foo, u8>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[cfg(feature = "serde")]
mod serde {
    use crate::id::test::int::Foo;
    use crate::Id;
    use serde_json;

    #[test]
    fn test_serialize_i128() {
        let id1: Id<Foo, i128> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_u128() {
        let id1: Id<Foo, u128> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_i64() {
        let id1: Id<Foo, i64> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_u64() {
        let id1: Id<Foo, u64> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_i32() {
        let id1: Id<Foo, i32> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_u32() {
        let id1: Id<Foo, u32> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_i16() {
        let id1: Id<Foo, i16> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_u16() {
        let id1: Id<Foo, u16> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_i8() {
        let id1: Id<Foo, i8> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_serialize_u8() {
        let id1: Id<Foo, u8> = Id::new(1);

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!("1".to_string(), got);
    }

    #[test]
    fn test_deserialize_i64() {
        let id1: Id<Foo, i64> = serde_json::from_str("1").unwrap();

        assert_eq!(id1.inner, 1);
    }

    #[test]
    fn test_deserialize_u64() {
        let id1: Id<Foo, u64> = serde_json::from_str("1").unwrap();

        assert_eq!(id1.inner, 1);
    }
    #[test]
    #[should_panic(expected = "expected i64")]
    fn test_deserialize_with_error() {
        let _: Id<Foo, i64> = serde_json::from_str("\"not i64 text\"").unwrap();
    }
}

#[cfg(feature = "fake")]
mod fake {
    use crate::id::test::int::Foo;
    use crate::Id;
    use fake::{Fake, Faker};

    #[test]
    fn test_fake_i128() {
        let id1: Id<Foo, i128> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_u128() {
        let id1: Id<Foo, u128> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }
    #[test]
    fn test_fake_i64() {
        let id1: Id<Foo, i64> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_u64() {
        let id1: Id<Foo, u64> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_i32() {
        let id1: Id<Foo, i32> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_u32() {
        let id1: Id<Foo, u32> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_i16() {
        let id1: Id<Foo, i16> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_u16() {
        let id1: Id<Foo, u16> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_i8() {
        let id1: Id<Foo, i8> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }

    #[test]
    fn test_fake_u8() {
        let id1: Id<Foo, u8> = Faker.fake();
        assert_ne!(id1.inner, 0);
    }
}
