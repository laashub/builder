table! {
    origin_public_keys(id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        revision -> Text,
        full_name -> Text,
        body -> Binary,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_secret_keys(id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        revision -> Text,
        full_name -> Text,
        body -> Binary,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_public_encryption_keys(id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        revision -> Text,
        full_name -> Text,
        body -> Binary,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}

table! {
    origin_private_encryption_keys(id) {
        id -> BigInt,
        origin_id -> BigInt,
        owner_id -> BigInt,
        name -> Text,
        revision -> Text,
        full_name -> Text,
        body -> Binary,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
    }
}