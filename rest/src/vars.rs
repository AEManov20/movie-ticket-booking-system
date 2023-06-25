use dotenv::dotenv;
use casey::upper;

// cheap fix
use stringify as STRINGIFY;

fn get(key: &str) -> Option<String> {
    dotenv().ok();
    match std::env::var(key) {
        Ok(v) => Some(v),
        Err(_) => None
    }
}

macro_rules! env {
    ($name:ident) => {
        pub fn $name() -> Option<String> {
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
    jwt_ticket_secret,
    gmail_user,
    gmail_password,
    server_protocol,
    server_domain,
    server_port
);