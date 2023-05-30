// @generated automatically by Diesel CLI.

diesel::table! {
    external_credentials (id) {
        id -> Uuid,
        #[max_length = 50]
        provider -> Varchar,
        #[max_length = 150]
        external_id -> Varchar,
        user_id -> Uuid,
    }
}

diesel::table! {
    halls (id) {
        id -> Uuid,
        number -> Int4,
        theatre_id -> Uuid,
        seat_data -> Json,
    }
}

diesel::table! {
    movie_reviews (id) {
        id -> Uuid,
        author_user_id -> Uuid,
        movie_id -> Uuid,
        #[max_length = 2500]
        content -> Nullable<Varchar>,
        rating -> Float8,
        created_at -> Timestamp,
        votes -> Int4,
    }
}

diesel::table! {
    movies (id) {
        id -> Uuid,
        #[max_length = 250]
        name -> Varchar,
        description -> Text,
        #[max_length = 250]
        genre -> Varchar,
        release_date -> Date,
        length -> Float8,
        votes -> Int4,
        #[max_length = 250]
        imdb_link -> Nullable<Varchar>,
        is_deleted -> Bool,
    }
}

diesel::table! {
    theatre_movies (id) {
        id -> Uuid,
        movie_id -> Uuid,
        theatre_id -> Uuid,
        hall_id -> Uuid,
        #[max_length = 50]
        subtitles_language -> Nullable<Varchar>,
        #[max_length = 50]
        audio_language -> Varchar,
        starting_time -> Timestamp,
        status -> Int4,
    }
}

diesel::table! {
    theatre_permissions (user_id, theatre_id) {
        user_id -> Uuid,
        theatre_id -> Uuid,
        can_manage_users -> Bool,
        can_manage_movies -> Bool,
        can_check_tickets -> Bool,
        can_manage_tickets -> Bool,
        is_theatre_owner -> Bool,
    }
}

diesel::table! {
    theatres (id) {
        id -> Uuid,
        #[max_length = 50]
        name -> Varchar,
        location_lat -> Float8,
        location_lon -> Float8,
        is_deleted -> Bool,
    }
}

diesel::table! {
    ticket_types (id) {
        id -> Uuid,
        #[sql_name = "type"]
        type_ -> Varchar,
        #[max_length = 50]
        movie_type -> Varchar,
        #[max_length = 300]
        description -> Nullable<Varchar>,
        theatre_id -> Uuid,
        #[max_length = 3]
        currency -> Varchar,
        price -> Float8,
    }
}

diesel::table! {
    tickets (id) {
        id -> Uuid,
        owner_user_id -> Uuid,
        theatre_movie_id -> Uuid,
        ticket_type_id -> Uuid,
        issuer_user_id -> Uuid,
        seat_row -> Int4,
        seat_column -> Int4,
        issued_at -> Timestamp,
        expires_at -> Timestamp,
        used -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 150]
        email -> Varchar,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 150]
        password_hash -> Nullable<Varchar>,
        created_at -> Timestamp,
        is_super_user -> Bool,
        is_activated -> Bool,
        is_deleted -> Bool,
    }
}

diesel::joinable!(external_credentials -> users (user_id));
diesel::joinable!(halls -> theatres (theatre_id));
diesel::joinable!(movie_reviews -> movies (movie_id));
diesel::joinable!(movie_reviews -> users (author_user_id));
diesel::joinable!(theatre_movies -> halls (hall_id));
diesel::joinable!(theatre_movies -> movies (movie_id));
diesel::joinable!(theatre_movies -> theatres (theatre_id));
diesel::joinable!(theatre_permissions -> theatres (theatre_id));
diesel::joinable!(theatre_permissions -> users (user_id));
diesel::joinable!(ticket_types -> theatres (theatre_id));
diesel::joinable!(tickets -> theatre_movies (theatre_movie_id));
diesel::joinable!(tickets -> ticket_types (ticket_type_id));
diesel::joinable!(tickets -> users (owner_user_id));

diesel::allow_tables_to_appear_in_same_query!(
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
