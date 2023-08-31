use rocket::{Rocket, Build, State, fairing::AdHoc};
use ldap3::LdapConn;
use jsonwebtoken::EncodingKey;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Config {
    ldap: Option<LdapConn>,
    jwt_key: Option<EncodingKey>,
    sql_conn: Option<Arc<Mutex<Connection>>>,
}

#[rocket::get("/")]
fn index(config: &State<Config>) -> &'static str {
    match &config.ldap {
        Some(_) => println!("INFO: LDAP available"),
        None => println!("ERROR: LDAP not set up"),
    }

    match &config.jwt_key {
        Some(_) => println!("INFO: JWT key available"),
        None => println!("ERROR: JWT key not set up"),
    }

    match &config.sql_conn {
        Some(_) => println!("INFO: SQL connection available"),
        None => println!("ERROR: SQL connection not set up"),
    }

    "Welcome to Permissions"
}

#[rocket::launch]
fn rocket() -> Rocket<Build> {
    let ldap_setup = match LdapConn::new("ldap://localhost:389") {
        Ok(conn) => {
            println!("INFO: LDAP setup succeeded.");
            Some(conn)
        },
        Err(err) => {
            println!("ERROR: LDAP setup failed: {}", err);
            None
        }
    };

    let jwt_key = Some(EncodingKey::from_secret("super_secret_key".as_ref()));
    println!("INFO: JWT Key set.");

    let sql_conn = match Connection::open("cache.db") {
        Ok(conn) => {
            println!("INFO: SQL connection succeeded.");
            Some(Arc::new(Mutex::new(conn)))
        },
        Err(err) => {
            println!("ERROR: SQL connection failed: {}", err);
            None
        }
    };

    let config = Config {
        ldap: ldap_setup,
        jwt_key,
        sql_conn,
    };

    rocket::build()
        .manage(config)
        .attach(AdHoc::on_ignite("Service Config", |rocket| async {
            println!("INFO: Service config attached.");
            rocket
        }))
        .mount("/", rocket::routes![index])
}