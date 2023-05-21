use crate::schema;
use crate::password;
use crate::vars::database_url;
use diesel::PgConnection;
use deadpool_diesel::postgres::{Runtime, Manager, Pool};
use rayon::prelude::*;
use diesel::connection::DefaultLoadingMode;

pub fn hash_mock_passwords(conn: &mut PgConnection) {
    use crate::model::User;
    use schema::users::dsl::*;
    use diesel::prelude::*;

    let hashed = users
        .load_iter::<User, DefaultLoadingMode>(conn)
        .unwrap()
        .collect::<Vec<_>>();

    let hashed = hashed
        .par_iter()
        .map(|x| {
            let Ok(x) = x else { return None };
            let Some(pass) = &x.password_hash else { return Some(User { password_hash: None, ..x.clone() }) };
            let Ok(hash) = password::hash(pass.as_bytes()) else { return Some(User { password_hash: None, ..x.clone() }) };

            println!("hashed: {}, {:?}", pass, hash);
            match password::verify(pass.as_bytes(), &hash) {
                true => Some(User { password_hash: Some(hash), ..x.clone() }),
                false => panic!("invalid")
            }
        })
        .collect::<Vec<_>>();

    for upd in hashed {
        let Some(upd) = upd else { continue; };
        println!("{:?}", upd);

        diesel::update(users)
            .filter(id.eq(upd.id))
            .set(password_hash.eq(upd.password_hash))
            .execute(conn)
            .unwrap();
    }
}

pub fn get_connection_pool() -> Pool {
    let url = database_url();
    // let manager = ConnectionManager::<PgConnection>::new(url);

    // Pool::builder()
    //     .max_size(5)
    //     .build(manager)
    //     .expect("Could not build connection pool")
    let manager = Manager::new(url, Runtime::Tokio1);
    Pool::builder(manager)
        .max_size(16)
        .build()
        .expect("Could not build connection pool")
}