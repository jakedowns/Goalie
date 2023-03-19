table! {
    auth_user (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        verified -> Bool,
        remember_me -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    games (id) {
        id -> Uuid,
        name -> Varchar,
        creator_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    rounds (id) {
        id -> Uuid,
        game_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    moves (id) {
        id -> Uuid,
        round_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    points (id) {
        id -> Uuid,
        move_id -> Uuid,
        value -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    times (id) {
        id -> Uuid,
        move_id -> Uuid,
        value -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    password_reset_requests (id) {
        id -> Uuid,
        user_id -> Uuid,
        token -> Varchar,
        expires_at -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    move_scores (id) {
        id -> Uuid,
        move_id -> Uuid,
        value -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    hidden_users (id) {
        id -> Uuid,
        user_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

joinable!(games -> auth_user (creator_id));
joinable!(moves -> rounds (round_id));
joinable!(points -> moves (move_id));
joinable!(rounds -> games (game_id));
joinable!(times -> moves (move_id));
joinable!(password_reset_requests -> auth_user (user_id));
joinable!(move_scores -> moves (move_id));
joinable!(hidden_users -> auth_user (user_id));

allow_tables_to_appear_in_same_query!(
    auth_user,
    games,
    rounds,
    moves,
    points,
    times,
    password_reset_requests,
    move_scores,
    hidden_users,
);

