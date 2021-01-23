table! {
    bins (id) {
        id -> Integer,
        author_id -> Integer,
        title -> Text,
        code -> Text,
    }
}

table! {
    likes (user_id, bin_id) {
        user_id -> Integer,
        bin_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        pseudo -> Text,
    }
}

joinable!(bins -> users (author_id));
joinable!(likes -> bins (bin_id));
joinable!(likes -> users (user_id));

allow_tables_to_appear_in_same_query!(bins, likes, users,);
