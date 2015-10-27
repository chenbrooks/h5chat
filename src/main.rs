extern crate iron;
extern crate persistent;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate urlencoded;
extern crate bodyparser;
extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;
extern crate crypto;
extern crate rand;


use rustorm::pool::ManagedPool;

use std::env;
use std::net::*;
use std::path::Path;

// Iron crates
use iron::prelude::*;
use iron::typemap::Key;
use router::Router;
use mount::Mount;
use staticfile::Static;
use persistent::Read as PersistRead;


// define this to use it with iron persistance cache plugin
pub struct AppDB;
impl Key for AppDB { type Value = ManagedPool; }

// import all helper macros
#[macro_use] mod macros ;
mod index;
mod user;



fn main() {
    let db_url: String = match env::var("H5CHAT_DATABASE_URL") {
        Ok(url) => {
            println!("{}", url);
            url
        },
        Err(_) => "postgres://postgres:123456@localhost:5432/test".to_string()
    };

    println!("connecting to postgres: {}", db_url);

    // here intro rustorm pool
    let pool = ManagedPool::init(&db_url, 4).unwrap();
    
    // router
    let mut router = Router::new();
    router.get("/", index::index);
    //router.get("json", json_test);
    router.post("/user/register", user::register);

    // mount
    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/p", Static::new(Path::new("./views/")));

    // middleware
    // ready to add middleware around mount entity
    let mut middleware = Chain::new(mount);
    // put db connect pool to persistance cache
    middleware.link(PersistRead::<AppDB>::both(pool));

    // http server
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    println!("listening on http://{}", host);
    // boot up
    Iron::new(middleware).http(host).unwrap();
}



