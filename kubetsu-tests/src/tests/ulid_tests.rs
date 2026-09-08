//! ULID as an ID inner type.
//!
//! Like UUID v7 it is time-ordered -- a 48-bit millisecond timestamp in the
//! leading bits of a `u128`, over which `Ulid` derives `Ord` -- so the same
//! question applies: does wrapping it in an ID disturb that.
//!
//! Only the core traits and serde are covered, because the other two
//! adapters cannot reach `Ulid` at all as things stand. `sqlx` implements
//! its type traits for no ULID type, and an application cannot add them
//! either, since both the traits and `Ulid` are foreign to it. `fake` does
//! implement `Dummy<Faker> for Ulid`, but against `ulid` 1.x, so it does
//! not apply to the 3.x type used here. Neither is a limitation of kubetsu,
//! and both would lift on the other crate's side.

use super::*;
use std::collections::{BTreeMap, HashMap};
use ulid::Ulid;

// As with UUID, the concrete form does not generate `PartialOrd`/`Ord`.
kubetsu::define_id!(
    #[derive(PartialOrd, Ord)]
    pub struct RecordId(Ulid);
);
kubetsu_serde::impl_serde!(RecordId(Ulid));

struct Record;
type MyRecordId = MyId<Record, Ulid>;

/// A ULID at a fixed instant, so ordering tests do not depend on the
/// wall clock.
fn ulid_at(millis: u64) -> Ulid {
    Ulid::from_parts(millis, 0)
}

#[test]
fn test_new_inner_and_from() {
    let raw = ulid_at(1);
    let id = RecordId::new(raw);
    assert_eq!(*id.inner(), raw);

    let from: RecordId = raw.into();
    assert_eq!(from, id);
}

#[test]
fn test_debug_delegates_to_inner() {
    let raw = ulid_at(1);
    assert_eq!(format!("{:?}", RecordId::new(raw)), format!("{:?}", raw));
}

#[test]
fn test_eq_clone_and_hash_key() {
    let id = RecordId::new(ulid_at(1));
    assert_eq!(id, id.clone());
    assert_ne!(id, RecordId::new(ulid_at(2)));

    let mut map = HashMap::new();
    map.insert(id.clone(), "first");
    assert_eq!(map.get(&id), Some(&"first"));
}

#[test]
fn test_ordering_survives_the_wrapper() {
    let ordered: Vec<RecordId> = (1..=5).map(|m| RecordId::new(ulid_at(m))).collect();

    let mut shuffled = ordered.clone();
    shuffled.reverse();
    shuffled.sort();

    assert_eq!(shuffled, ordered);
}

#[test]
fn test_ordering_generic_form() {
    let ordered: Vec<MyRecordId> = (1..=5).map(|m| MyRecordId::new(ulid_at(m))).collect();

    let mut shuffled = ordered.clone();
    shuffled.reverse();
    shuffled.sort();

    assert_eq!(shuffled, ordered);
}

#[test]
fn test_btree_key_iterates_in_timestamp_order() {
    let mut map = BTreeMap::new();
    map.insert(RecordId::new(ulid_at(3)), "third");
    map.insert(RecordId::new(ulid_at(1)), "first");
    map.insert(RecordId::new(ulid_at(2)), "second");

    assert_eq!(
        map.into_values().collect::<Vec<_>>(),
        vec!["first", "second", "third"]
    );
}

#[test]
fn test_serde_round_trip() {
    let id = RecordId::new(ulid_at(1));

    // `Ulid` serializes as its 26-character Crockford base32 form, so
    // the ID does too.
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, format!("\"{}\"", id.inner()));
    assert_eq!(json.len(), 28); // 26 characters plus the two quotes

    let back: RecordId = serde_json::from_str(&json).unwrap();
    assert_eq!(back, id);
}

#[test]
fn test_serde_as_json_map_key() {
    // The string form is what allows an ID to be a JSON object key.
    let id = RecordId::new(ulid_at(1));
    let mut map = HashMap::new();
    map.insert(id.clone(), 1);

    let json = serde_json::to_string(&map).unwrap();
    assert_eq!(json, format!("{{\"{}\":1}}", id.inner()));

    let back: HashMap<RecordId, i32> = serde_json::from_str(&json).unwrap();
    assert_eq!(back.get(&id), Some(&1));
}
