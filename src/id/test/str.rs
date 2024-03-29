use crate::Id;
use std::collections::HashMap;

struct Foo {}

#[test]
fn test_str_partial_eq() {
    let id1: Id<Foo, String> = Id::new("1".to_string());
    let id2: Id<Foo, String> = Id::new("1".to_string());
    let id3: Id<Foo, String> = Id::new("2".to_string());

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_str_inner() {
    let id1: Id<Foo, String> = Id::new("1".to_string());
    assert_eq!(id1.inner(), "1");
    assert_ne!(id1.inner(), "");
}

#[test]
fn test_str_clone() {
    let id1: Id<Foo, String> = Id::new("1".to_string());
    assert_eq!(id1.clone(), id1);
}

#[test]
fn test_hash_key_str() {
    let id1: Id<Foo, String> = Id::new("1".to_string());
    let mut hash: HashMap<Id<Foo, String>, bool> = HashMap::new();
    hash.insert(id1.clone(), true);
    assert_eq!(hash.get(&id1), Some(&true));
}

#[cfg(feature = "serde")]
mod serde {
    use crate::id::test::str::Foo;
    use crate::Id;

    #[test]
    fn test_serialize() {
        let id1: Id<Foo, String> = Id::new("1".to_string());

        let got = serde_json::to_string(&id1).unwrap();
        assert_eq!(got, "\"1\"")
    }

    #[test]
    fn test_deserialize() {
        let id1: Id<Foo, String> = serde_json::from_str("\"kubetsu\"").unwrap();

        assert_eq!(id1.inner(), "kubetsu");
    }
}

#[cfg(feature = "fake")]
mod fake {
    use crate::id::test::str::Foo;
    use crate::Id;
    use fake::{Fake, Faker};

    #[test]
    fn test_fake() {
        let id1: Id<Foo, String> = Faker.fake();
        assert_ne!(id1.inner, "");
    }
}
