use dotenv::dotenv;
use casey::upper;

// cheap fix
use stringify as STRINGIFY;

fn get(key: &str) -> String {
    dotenv().ok();
    std::env::var(key).expect(&format!("{} env must be set.", key))
}

macro_rules! env {
    ($name:ident) => {
        pub fn $name() -> String {
            get(upper!(stringify!($name)))
        }
    };
}

macro_rules! envs {
    ($( $name:ident ),*) => {
        $(
            env!($name);
        )*
    };
}

envs!(
    database_url,
    hash_secret,
    jwt_user_secret,
    jwt_email_secret,
    jwt_ticket_secret
);