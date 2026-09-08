//! Integration tests: all adapter crates combined on the same types.

#[cfg(test)]
mod tests {
    // --- Concrete form ---

    kubetsu::define_id!(
        pub struct UserId(i64);
    );
    kubetsu_serde::impl_serde!(UserId(i64));
    kubetsu_fake::impl_fake!(UserId(i64));
    kubetsu_sqlx::impl_sqlx!(UserId(i64));

    kubetsu::define_id!(
        pub struct ItemId(String);
    );
    kubetsu_serde::impl_serde!(ItemId(String));
    kubetsu_fake::impl_fake!(ItemId(String));
    kubetsu_sqlx::impl_sqlx!(ItemId(String));

    // --- Generic form ---

    kubetsu::define_id!(
        pub struct MyId<T, U>;
    );
    kubetsu_serde::impl_serde!(MyId<T, U>);
    kubetsu_fake::impl_fake!(MyId<T, U>);
    kubetsu_sqlx::impl_sqlx!(MyId<T, U>);

    struct User;
    type MyUserId = MyId<User, i64>;

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

    mod sqlx_tests {
        use super::*;
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        use sqlx::{FromRow, SqlitePool};

        async fn get_db_conn() -> Result<SqlitePool, sqlx::Error> {
            let connect_info = SqliteConnectOptions::new();
            let pool = SqlitePoolOptions::new()
                .connect_with(connect_info)
                .await
                .unwrap();
            Ok(pool)
        }

        #[derive(FromRow)]
        struct Row {
            id: UserId,
        }

        #[tokio::test]
        async fn test_combined_sqlx_concrete() {
            let conn = get_db_conn().await.unwrap();
            let mut tx = conn.begin().await.unwrap();
            let row: Row = sqlx::query_as("SELECT 1 as id")
                .fetch_one(&mut *tx)
                .await
                .unwrap();
            assert_eq!(*row.id.inner(), 1);
        }

        #[derive(FromRow)]
        struct GenericRow {
            id: MyUserId,
        }

        #[tokio::test]
        async fn test_combined_sqlx_generic() {
            let conn = get_db_conn().await.unwrap();
            let mut tx = conn.begin().await.unwrap();
            let row: GenericRow = sqlx::query_as("SELECT 1 as id")
                .fetch_one(&mut *tx)
                .await
                .unwrap();
            assert_eq!(*row.id.inner(), 1);
        }
    }

    /// UUID v7 as an ID inner type.
    ///
    /// v7 is time-ordered: its 48-bit big-endian Unix-millisecond timestamp is
    /// the first field, and `Uuid` derives `Ord` over the raw `[u8; 16]`, so
    /// byte order is timestamp order. Wrapping it in an ID must not disturb
    /// that, which is what most of these tests are about.
    mod uuid_tests {
        use super::*;
        use std::collections::{BTreeMap, HashMap};
        use uuid::{NoContext, Timestamp, Uuid};

        // The concrete form does not generate `PartialOrd`/`Ord`; derive them,
        // which is the documented path and what makes v7's ordering usable.
        kubetsu::define_id!(
            #[derive(PartialOrd, Ord)]
            pub struct EventId(Uuid);
        );
        kubetsu_serde::impl_serde!(EventId(Uuid));
        kubetsu_fake::impl_fake!(EventId(Uuid));

        // No `impl_sqlx!` here on purpose. `sqlx::Any` has no `Uuid` support,
        // and the concrete form emits its `Any` implementations unconditionally,
        // so `impl_sqlx!(EventId(Uuid))` stops compiling as soon as anything in
        // the build turns on kubetsu-sqlx's `any` feature -- which Cargo feature
        // unification can do from another crate entirely. The generic form
        // bounds each implementation on the inner type, so it just skips `Any`;
        // the sqlx test below uses it.
        struct Event;
        type MyEventId = MyId<Event, Uuid>;

        /// A v7 UUID at a fixed instant, so ordering tests do not depend on the
        /// wall clock. Two v7 values within the same millisecond differ only in
        /// random bits, which would make a `Instant::now()`-based test flaky.
        fn v7_at(secs: u64) -> Uuid {
            Uuid::new_v7(Timestamp::from_unix(NoContext, secs, 0))
        }

        #[test]
        fn test_new_inner_and_from() {
            let raw = v7_at(1);
            let id = EventId::new(raw);
            assert_eq!(*id.inner(), raw);

            let from: EventId = raw.into();
            assert_eq!(from, id);
        }

        #[test]
        fn test_debug_delegates_to_inner() {
            let raw = v7_at(1);
            assert_eq!(format!("{:?}", EventId::new(raw)), format!("{:?}", raw));
        }

        #[test]
        fn test_eq_and_clone() {
            let id = EventId::new(v7_at(1));
            assert_eq!(id, id.clone());
            assert_ne!(id, EventId::new(v7_at(2)));
        }

        #[test]
        fn test_hash_key() {
            let id = EventId::new(v7_at(1));
            let mut map = HashMap::new();
            map.insert(id.clone(), "first");
            assert_eq!(map.get(&id), Some(&"first"));
        }

        #[test]
        fn test_v7_ordering_survives_the_wrapper() {
            // Built in timestamp order, then reversed, so sorting has work to do
            // without depending on a random shuffle.
            let ordered: Vec<EventId> = (1..=5).map(|s| EventId::new(v7_at(s))).collect();

            let mut shuffled = ordered.clone();
            shuffled.reverse();
            shuffled.sort();

            assert_eq!(shuffled, ordered);
        }

        #[test]
        fn test_v7_ordering_generic_form() {
            let ordered: Vec<MyEventId> = (1..=5).map(|s| MyEventId::new(v7_at(s))).collect();

            let mut shuffled = ordered.clone();
            shuffled.reverse();
            shuffled.sort();

            assert_eq!(shuffled, ordered);
        }

        #[test]
        fn test_serde_round_trip() {
            let id = EventId::new(v7_at(1));

            // `Uuid` serializes as a hyphenated string, so the ID does too.
            let json = serde_json::to_string(&id).unwrap();
            assert_eq!(json, format!("\"{}\"", id.inner()));

            let back: EventId = serde_json::from_str(&json).unwrap();
            assert_eq!(back, id);
        }

        #[test]
        fn test_serde_as_json_map_key() {
            // A string-shaped serialization is what lets an ID be a JSON object
            // key; `impl_serde!` delegates, so this holds for `Uuid` too.
            let id = EventId::new(v7_at(1));
            let mut map = HashMap::new();
            map.insert(id.clone(), 1);

            let json = serde_json::to_string(&map).unwrap();
            assert_eq!(json, format!("{{\"{}\":1}}", id.inner()));

            let back: HashMap<EventId, i32> = serde_json::from_str(&json).unwrap();
            assert_eq!(back.get(&id), Some(&1));
        }

        #[test]
        fn test_fake_with_faker_gives_an_arbitrary_uuid() {
            use fake::{Fake, Faker};

            // fake's `Dummy<Faker> for Uuid` fills all 128 bits at random, so
            // the default config yields an arbitrary UUID rather than any
            // particular version -- it does not even set the version bits. Ask
            // for a version explicitly, as below.
            //
            // Assert that rather than merely that two draws differ, so the
            // claim fails loudly if fake ever starts producing a real version
            // here: the version nibble is uniform over 16 values, so seeing
            // only one across 20 draws has probability 16 * 16^-20.
            let versions: std::collections::HashSet<usize> = (0..20)
                .map(|_| Faker.fake::<EventId>().inner().get_version_num())
                .collect();
            assert!(
                versions.len() > 1,
                "expected unset version bits to vary, saw only {versions:?}"
            );

            let generic: MyEventId = Faker.fake();
            assert_ne!(generic, Faker.fake::<MyEventId>());
        }

        #[test]
        fn test_fake_generates_a_v7() {
            use fake::Fake;
            use fake::uuid::UUIDv7;

            // `impl_fake!` forwards the config to the inner type, so fake's
            // UUID version configs reach an ID directly.
            //
            // What reaches it is a structurally valid v7, not a chronological
            // one: fake draws the timestamp at random rather than from the
            // clock, so generated values do not sort into generation order the
            // way real v7s do. Build them from explicit timestamps, as the
            // ordering tests above do, when order is what matters.
            let id: EventId = UUIDv7.fake();
            assert_eq!(id.inner().get_version_num(), 7);

            let generic: MyEventId = UUIDv7.fake();
            assert_eq!(generic.inner().get_version_num(), 7);
        }

        #[test]
        fn test_fake_generates_a_v7_through_derive() {
            use fake::uuid::UUIDv7;
            use fake::{Dummy, Fake, Faker};

            #[derive(Dummy)]
            struct Event {
                #[dummy(faker = "UUIDv7")]
                id: EventId,
                #[dummy(faker = "UUIDv7")]
                generic_id: MyEventId,
            }

            let event: Event = Faker.fake();
            assert_eq!(event.id.inner().get_version_num(), 7);
            assert_eq!(event.generic_id.inner().get_version_num(), 7);
        }

        #[tokio::test]
        async fn test_sqlx_sqlite_round_trip() {
            use sqlx::FromRow;
            use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};

            #[derive(FromRow)]
            struct Row {
                id: MyEventId,
            }

            let pool = SqlitePoolOptions::new()
                .connect_with(SqliteConnectOptions::new())
                .await
                .unwrap();
            let mut tx = pool.begin().await.unwrap();

            sqlx::query("CREATE TABLE events (id BLOB PRIMARY KEY)")
                .execute(&mut *tx)
                .await
                .unwrap();

            let id = MyEventId::new(v7_at(1));
            sqlx::query("INSERT INTO events (id) VALUES (?)")
                .bind(&id)
                .execute(&mut *tx)
                .await
                .unwrap();

            let row: Row = sqlx::query_as("SELECT id FROM events")
                .fetch_one(&mut *tx)
                .await
                .unwrap();

            assert_eq!(row.id, id);
        }

        #[test]
        fn test_btree_key_iterates_in_timestamp_order() {
            let mut map = BTreeMap::new();
            map.insert(EventId::new(v7_at(3)), "third");
            map.insert(EventId::new(v7_at(1)), "first");
            map.insert(EventId::new(v7_at(2)), "second");

            assert_eq!(
                map.into_values().collect::<Vec<_>>(),
                vec!["first", "second", "third"]
            );
        }
    }

    /// ULID as an ID inner type. See `tests/ulid_tests.rs`.
    mod ulid_tests;
}
