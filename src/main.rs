extern crate hyper;
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
extern crate cookie;
extern crate rand;
extern crate jsonway;

extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
#[macro_use] extern crate lazy_static;
extern crate toml;

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

use std::default::Default;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
use redis::Commands;

// import all helper macros
#[macro_use] mod macros ;
//mod helper;
mod config;
use config::ConfigManager;

mod midware;
mod dbdesign;
mod index;
mod user;
mod chatroom;

// define this to use it with iron persistance cache plugin
pub struct AppDB;
impl Key for AppDB { type Value = ManagedPool; }

pub type RedisPool = Pool<RedisConnectionManager>;
pub struct AppRedis;
impl Key for AppRedis { type Value = RedisPool; }

use midware::CheckLogin;

fn main() {
    // create db pool
    let db_url = ConfigManager::get_config_str("h5chat", "pg_db_url");
    println!("connecting to postgres: {}", db_url);
    let pool = ManagedPool::init(&db_url, 4).unwrap();
    // create redis pool
    let redis_url = ConfigManager::get_config_str("h5chat", "redis_url");
    println!("connecting to redis: {}", redis_url);
    let redis_config = Default::default();
    let manager = RedisConnectionManager::new(&redis_url[..]).unwrap();
    let redis_pool = r2d2::Pool::new(redis_config, manager).unwrap();
    
    // router
    let mut router = Router::new();
    router.get("/", index::index);
    router.post("/user/register", user::register);
    router.post("/user/login", user::login);
    router.get("/user/logout", user::logout);
    
    router.post("/board/room/create", chatroom::create);
    router.get("/board/room/myrooms", chatroom::myrooms);

    // mount
    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static/", Static::new(Path::new("./static/")));
    mount.mount("/page/", Static::new(Path::new("./views/")));

    // middleware
    // ready to add middleware around mount entity
    let mut middleware = Chain::new(mount);
    // put db connect pool to persistance cache
    middleware.link(PersistRead::<AppDB>::both(pool));
    middleware.link(PersistRead::<AppRedis>::both(redis_pool));
    middleware.link_before(CheckLogin);
    middleware.link_after(CheckLogin);
    //middleware.link_after(HandlebarsEngine::new("./views/", ".html"));
    
    // http server
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    println!("listening on http://{}", host);
    // boot up
    Iron::new(middleware).http(host).unwrap();
}



