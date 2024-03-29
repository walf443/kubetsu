use crate::Id;

struct Foo {}

#[test]
fn test_partial_eq_f64() {
    let id1: Id<Foo, f64> = Id::new(1.0);
    let id2: Id<Foo, f64> = Id::new(1.0);
    let id3: Id<Foo, f64> = Id::new(2.0);

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_partial_eq_f32() {
    let id1: Id<Foo, f32> = Id::new(1.0);
    let id2: Id<Foo, f32> = Id::new(1.0);
    let id3: Id<Foo, f32> = Id::new(2.0);

    assert_eq!(id1, id2);
    assert_ne!(id1, id3);
}

#[test]
fn test_inner_f64() {
    let id1: Id<Foo, f64> = Id::new(1.0);
    assert_eq!(id1.inner(), &1.0);
}

#[test]
fn test_inner_f32() {
    let id1: Id<Foo, f32> = Id::new(1.0);
    assert_eq!(id1.inner(), &1.0);
}

#[test]
fn test_clone_f64() {
    let id1: Id<Foo, f64> = Id::new(1.0);
    assert_eq!(id1.clone(), id1);
}

#[test]
fn test_clone_f32() {
    let id1: Id<Foo, f32> = Id::new(1.0);
    assert_eq!(id1.clone(), id1);
}

#[cfg(feature = "serde")]
mod serde {
    use crate::id::test::float::Foo;
    use crate::Id;

    #[test]
    fn test_serialize_f32() {
        let id1: Id<Foo, f32> = Id::new(1.0);

        let got = serde_json::to_string(&id1).unwrap();

        assert_eq!(got, "1.0");
    }

    #[test]
    fn test_serialize_f64() {
        let id1: Id<Foo, f64> = Id::new(1.0);

        let got = serde_json::to_string(&id1).unwrap();

        assert_eq!(got, "1.0");
    }

    #[test]
    fn test_deserialize_f32() {
        let id1: Id<Foo, f32> = serde_json::from_str("1.0").unwrap();

        assert_eq!(id1.inner, 1.0);
    }
    #[test]
    fn test_deserialize_f64() {
        let id1: Id<Foo, f64> = serde_json::from_str("1.0").unwrap();

        assert_eq!(id1.inner, 1.0);
    }

    #[test]
    #[should_panic(expected = "expected f64")]
    fn test_deserialize_with_error() {
        let _: Id<Foo, f64> = serde_json::from_str("\"not i64 text\"").unwrap();
    }
}

#[cfg(feature = "fake")]
mod fake {
    use crate::id::test::float::Foo;
    use crate::Id;
    use fake::{Fake, Faker};

    #[test]
    fn test_fake_f64() {
        let id1: Id<Foo, f64> = Faker.fake();
        assert_ne!(id1.inner, 0.0);
    }

    #[test]
    fn test_fake_f32() {
        let id1: Id<Foo, f32> = Faker.fake();
        assert_ne!(id1.inner, 0.0);
    }
}
