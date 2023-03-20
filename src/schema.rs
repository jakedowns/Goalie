// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Nullable<Integer>,
        name -> Text,
        creator_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    hidden_users (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    move_scores (id) {
        id -> Nullable<Integer>,
        move_type -> Integer,
        value -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    moves (id) {
        id -> Nullable<Integer>,
        player_id -> Integer,
        round_id -> Nullable<Integer>,
        game_id -> Integer,
        points_id -> Nullable<Integer>,
        times_id -> Nullable<Integer>,
        move_type -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    password_reset_requests (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        token -> Text,
        expires_at -> Timestamp,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    points (id) {
        id -> Nullable<Integer>,
        move_id -> Integer,
        value -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    rounds (id) {
        id -> Nullable<Integer>,
        game_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    times (id) {
        id -> Nullable<Integer>,
        move_id -> Integer,
        value -> Timestamp,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        username -> Text,
        email -> Text,
        password_hash -> Text,
        verified -> Nullable<Bool>,
        remember_me -> Nullable<Bool>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    games,
    hidden_users,
    move_scores,
    moves,
    password_reset_requests,
    points,
    rounds,
    times,
    users,
);
