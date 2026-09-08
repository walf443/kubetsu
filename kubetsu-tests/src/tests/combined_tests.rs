//! Every adapter exercised on one ID, in each macro form.

use super::*;

#[test]
fn test_combined_concrete() {
    use fake::{Fake, Faker};

    let id = UserId::new(42);

    // serde
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "42");
    let deserialized: UserId = serde_json::from_str(&json).unwrap();
    assert_eq!(*deserialized.inner(), 42);

    // fake
    let _fake_id: UserId = Faker.fake();

    // core traits
    let cloned = id.clone();
    assert_eq!(id, cloned);
}

#[test]
fn test_combined_generic() {
    use fake::{Fake, Faker};

    let id = MyUserId::new(42);

    // serde
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "42");
    let deserialized: MyUserId = serde_json::from_str(&json).unwrap();
    assert_eq!(*deserialized.inner(), 42);

    // fake
    let _fake_id: MyUserId = Faker.fake();

    // core traits
    let cloned = id.clone();
    assert_eq!(id, cloned);
}
