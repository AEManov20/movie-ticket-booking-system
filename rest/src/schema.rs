// @generated automatically by Diesel CLI.

diesel::table! {
    external_credentials (id) {
        id -> Uuid,
        provider -> Varchar,
        external_id -> Varchar,
        user_id -> Uuid,
    }
}

diesel::table! {
    halls (id) {
        id -> Uuid,
        theatre_id -> Uuid,
        name -> Varchar,
        seat_data -> Json,
    }
}

diesel::table! {
    languages (id) {
        id -> Uuid,
        code -> Bpchar,
        name -> Varchar,
    }
}

diesel::table! {
    movie_reviews (id) {
        id -> Uuid,
        author_user_id -> Uuid,
        movie_id -> Uuid,
        content -> Nullable<Varchar>,
        rating -> Float8,
        created_at -> Timestamp,
        votes -> Int4,
    }
}

diesel::table! {
    movies (id) {
        id -> Uuid,
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
    theatre_roles (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    theatre_screenings (id) {
        id -> Uuid,
        movie_id -> Uuid,
        theatre_id -> Uuid,
        hall_id -> Uuid,
        subtitles_language -> Nullable<Uuid>,
        audio_language -> Uuid,
        starting_time -> Timestamp,
        is_3d -> Bool,
        status -> Int4,
    }
}

diesel::table! {
    theatres (id) {
        id -> Uuid,
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
        description -> Nullable<Varchar>,
        theatre_id -> Uuid,
        currency -> Varchar,
        price -> Float8,
    }
}

diesel::table! {
    tickets (id) {
        id -> Uuid,
        owner_user_id -> Uuid,
        theatre_screening_id -> Uuid,
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
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        username -> Varchar,
        password_hash -> Nullable<Varchar>,
        created_at -> Timestamp,
        is_super_user -> Bool,
        is_activated -> Bool,
        is_deleted -> Bool,
    }
}

diesel::table! {
    users_theatre_roles (user_id, role_id, theatre_id) {
        user_id -> Uuid,
        role_id -> Uuid,
        theatre_id -> Uuid,
    }
}

diesel::joinable!(external_credentials -> users (user_id));
diesel::joinable!(halls -> theatres (theatre_id));
diesel::joinable!(movie_reviews -> movies (movie_id));
diesel::joinable!(movie_reviews -> users (author_user_id));
diesel::joinable!(theatre_screenings -> halls (hall_id));
diesel::joinable!(theatre_screenings -> movies (movie_id));
diesel::joinable!(theatre_screenings -> theatres (theatre_id));
diesel::joinable!(ticket_types -> theatres (theatre_id));
diesel::joinable!(tickets -> theatre_screenings (theatre_screening_id));
diesel::joinable!(tickets -> ticket_types (ticket_type_id));
diesel::joinable!(tickets -> users (owner_user_id));
diesel::joinable!(users_theatre_roles -> theatre_roles (role_id));
diesel::joinable!(users_theatre_roles -> theatres (theatre_id));
diesel::joinable!(users_theatre_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    external_credentials,
    halls,
    languages,
    movie_reviews,
    movies,
    theatre_roles,
    theatre_screenings,
    theatres,
    ticket_types,
    tickets,
    users,
    users_theatre_roles,
);
