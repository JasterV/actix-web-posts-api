table! {
    posts (uuid) {
        uuid -> Uuid,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
