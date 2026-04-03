use std::collections::HashMap;

// --- Concrete form ---

crate::define_id!(
    pub struct UserId(i64);
);
crate::define_id!(
    pub struct ItemId(String);
);

// --- Generic form ---

crate::define_id!(
    pub struct MyId<T, U>;
);

struct User;
struct Item;
type MyUserId = MyId<User, i64>;
type MyItemId = MyId<Item, i64>;

#[test]
fn test_new_and_inner() {
    let id = UserId::new(42);
    assert_eq!(*id.inner(), 42);
}

#[test]
fn test_from() {
    let id: UserId = 42.into();
    assert_eq!(*id.inner(), 42);
}

#[test]
fn test_eq() {
    let a = UserId::new(1);
    let b = UserId::new(1);
    let c = UserId::new(2);
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn test_clone() {
    let a = UserId::new(1);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn test_debug() {
    let id = UserId::new(42);
    assert_eq!(format!("{:?}", id), "42");
}

#[test]
fn test_hash() {
    let mut map = HashMap::new();
    let id = UserId::new(1);
    map.insert(id.clone(), "user");
    assert_eq!(map.get(&id), Some(&"user"));
}

#[test]
fn test_string_id() {
    let id = ItemId::new("abc".to_string());
    assert_eq!(id.inner(), "abc");
}

mod generic_tests {
    use super::*;

    #[test]
    fn test_new_and_inner() {
        let id = MyUserId::new(42);
        assert_eq!(*id.inner(), 42);
    }

    #[test]
    fn test_from() {
        let id: MyUserId = 42.into();
        assert_eq!(*id.inner(), 42);
    }

    #[test]
    fn test_eq() {
        let a = MyUserId::new(1);
        let b = MyUserId::new(1);
        assert_eq!(a, b);
    }

    #[test]
    fn test_type_distinction() {
        let _user_id = MyUserId::new(1);
        let _item_id = MyItemId::new(1);
    }

    #[test]
    fn test_clone() {
        let a = MyUserId::new(1);
        let b = a.clone();
        assert_eq!(a, b);
    }

    #[test]
    fn test_debug() {
        let id = MyUserId::new(42);
        assert_eq!(format!("{:?}", id), "42");
    }

    #[test]
    fn test_hash() {
        let mut map = HashMap::new();
        let id = MyUserId::new(1);
        map.insert(id.clone(), "user");
        assert_eq!(map.get(&id), Some(&"user"));
    }
}
