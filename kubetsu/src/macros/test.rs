use std::collections::{BTreeMap, HashMap, HashSet};

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
fn test_eq_is_reflexive() {
    // Documents what `Eq` buys an ID: reflexive equality, and deduplication in
    // a `HashSet`. This is not the regression guard for the inner-type
    // assertion -- `UserId`'s inner type is `i64`, which is always `Eq`, so
    // this passes with or without it. The guard is the `compile_fail` doctest
    // on `define_id!`.
    let id = UserId::new(1);
    assert_eq!(id, id.clone());

    let mut set = HashSet::new();
    set.insert(id.clone());
    set.insert(id.clone());
    assert_eq!(set.len(), 1);
}

#[test]
fn test_string_id() {
    let id = ItemId::new("abc".to_string());
    assert_eq!(id.inner(), "abc");
}

mod generic_tests {
    use super::*;
    use std::cmp::Ordering;

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

    #[test]
    fn test_eq_is_conditional() {
        // The positive half only: an `Eq` inner type yields an `Eq` ID. This
        // passes even if the bound were weakened, so the regression guard is
        // the negative half -- a `compile_fail` doctest on `define_id!`, since
        // absence of a trait cannot be asserted at runtime.
        fn requires_eq<T: Eq>() {}
        requires_eq::<MyUserId>();
    }

    #[test]
    fn test_ord() {
        let a = MyUserId::new(1);
        let b = MyUserId::new(2);
        assert!(a < b);
        assert_eq!(a.cmp(&b), Ordering::Less);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
    }

    #[test]
    fn test_ord_as_btree_key() {
        let mut map = BTreeMap::new();
        map.insert(MyUserId::new(2), "b");
        map.insert(MyUserId::new(1), "a");
        assert_eq!(map.into_values().collect::<Vec<_>>(), vec!["a", "b"]);
    }

    #[test]
    fn test_partial_ord_without_ord() {
        // `f64` is `PartialOrd` but not `Ord`, so the ID follows: comparable,
        // but not usable where `Ord` is required.
        type MyFloatId = MyId<User, f64>;

        let a = MyFloatId::new(1.0);
        let b = MyFloatId::new(2.0);
        assert!(a < b);
        assert_eq!(a.partial_cmp(&b), Some(Ordering::Less));
        assert_eq!(MyFloatId::new(f64::NAN).partial_cmp(&a), None);
    }
}
