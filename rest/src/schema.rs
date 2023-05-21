// @generated automatically by Diesel CLI.

diesel::table! {
    email_confirmations (id) {
        id -> Int4,
        key -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    external_credentials (id) {
        id -> Int4,
        provider -> Varchar,
        external_id -> Varchar,
        user_id -> Int4,
    }
}

diesel::table! {
    halls (id) {
        id -> Int4,
        number -> Int4,
        theatre_id -> Int4,
        seat_data -> Json,
    }
}

diesel::table! {
    movie_reviews (id) {
        id -> Int4,
        author_user_id -> Int4,
        movie_id -> Int4,
        content -> Nullable<Varchar>,
        rating -> Float8,
    }
}

diesel::table! {
    movies (id) {
        id -> Int4,
        name -> Varchar,
        description -> Text,
        genre -> Varchar,
        release_date -> Date,
        length -> Float8,
        imdb_link -> Nullable<Varchar>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    theatre_movies (id) {
        id -> Int4,
        movie_id -> Int4,
        hall_id -> Int4,
        subtitles_language -> Nullable<Varchar>,
        audio_language -> Varchar,
        starting_time -> Timestamp,
    }
}

diesel::table! {
    theatre_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        theatre_id -> Int4,
        can_manage_users -> Bool,
        can_manage_movies -> Bool,
        can_check_tickets -> Bool,
        can_manage_tickets -> Bool,
        is_theatre_owner -> Bool,
    }
}

diesel::table! {
    theatres (id) {
        id -> Int4,
        name -> Varchar,
        location_lat -> Float8,
        location_lon -> Float8,
        is_deleted -> Bool,
    }
}

diesel::table! {
    ticket_types (id) {
        id -> Int4,
        #[sql_name = "type"]
        type_ -> Varchar,
        movie_type -> Varchar,
        description -> Nullable<Varchar>,
        theatre_id -> Int4,
        currency -> Varchar,
        price -> Float8,
    }
}

diesel::table! {
    tickets (id) {
        id -> Int4,
        owner_user_id -> Int4,
        theatre_movie_id -> Int4,
        ticket_type_id -> Int4,
        issuer_user_id -> Nullable<Int4>,
        seat_row -> Int4,
        seat_column -> Int4,
        expires_at -> Timestamp,
        used -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Nullable<Varchar>,
        is_super_user -> Bool,
        is_activated -> Bool,
        is_deleted -> Bool,
    }
}

diesel::joinable!(email_confirmations -> users (user_id));
diesel::joinable!(external_credentials -> users (user_id));
diesel::joinable!(halls -> theatres (theatre_id));
diesel::joinable!(movie_reviews -> movies (movie_id));
diesel::joinable!(movie_reviews -> users (author_user_id));
diesel::joinable!(theatre_movies -> halls (hall_id));
diesel::joinable!(theatre_movies -> movies (movie_id));
diesel::joinable!(theatre_permissions -> theatres (theatre_id));
diesel::joinable!(theatre_permissions -> users (user_id));
diesel::joinable!(ticket_types -> theatres (theatre_id));
diesel::joinable!(tickets -> theatre_movies (theatre_movie_id));
diesel::joinable!(tickets -> ticket_types (ticket_type_id));
diesel::joinable!(tickets -> users (owner_user_id));

diesel::allow_tables_to_appear_in_same_query!(
    email_confirmations,
    external_credentials,
    halls,
    movie_reviews,
    movies,
    theatre_movies,
    theatre_permissions,
    theatres,
    ticket_types,
    tickets,
    users,
);
