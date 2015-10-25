extern crate iron;
extern crate persistent;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustorm;
extern crate uuid;
extern crate chrono;
extern crate rustc_serialize;
extern crate r2d2;
extern crate r2d2_postgres;


use rustorm::dao::{Dao, IsDao};
use rustorm::pool::ManagedPool;
use rustorm::table::{Table, Column};
use rustorm::table::IsTable;
use rustorm::pool::Platform;
use rustorm::platform::postgres::Postgres;
use rustorm::query::Query;
use rustorm::query::Equality;
use rustorm::database::DbError;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager};


use std::env;
use std::net::*;
use std::path::Path;
use std::collections::BTreeMap;
use uuid::Uuid;

// Json crates
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};

// Iron crates
use iron::prelude::*;
use iron::status;
use iron::typemap::Key;
use router::Router;
use mount::Mount;
use staticfile::Static;
use persistent::Read as PersistRead;


// define this to use it with iron persistance cache plugin
pub type PostgresPool = Pool<PostgresConnectionManager>;
pub struct AppDB;
impl Key for AppDB { type Value = PostgresPool; }









#[derive(Debug, Clone)]
pub struct Product {
    pub pid: String,
    pub name: Option<String>,
    pub desc: Option<String>
}

impl IsDao for Product {
    fn from_dao(dao: &Dao) -> Self {
        Product {
            pid: dao.get("pid"),
            name: dao.get_opt("name"),
            desc: dao.get_opt("desc")
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set("pid", &self.pid);
        match self.name {
            Some(ref _value) => dao.set("name", _value),
            None => dao.set_null("name"),
        }
        match self.desc {
            Some(ref _value) => dao.set("desc", _value),
            None => dao.set_null("desc"),
        }
        dao
    }
}

impl IsTable for Product{
    fn table() -> Table {
        Table {
            schema: "public".to_string(),
            name: "product".to_string(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![],
            is_view: false,
        }
    }
}


fn index(req: &mut Request) -> IronResult<Response> {
    let inner_pool = req.get::<PersistRead<AppDB>>().unwrap();
    let db_wr = match inner_pool.get() {
        Ok(conn) => {
            let pg = Postgres::with_pooled_connection(conn);
            Ok(Platform::Postgres(pg))
        }   
        Err(e) => {
            Err(DbError::new(&format!("Unable to connect due to {}", e)))
        }   
    };
    let db = db_wr.unwrap();

    //let mut Arc(pool) = ManagedPool::Postgres(inner_pool);
    // let db = pool.connect().unwrap();
/*
    let p: Product = Query::select_all()
                            .from_table("public.product")
                            .collect_one(db.as_ref())
                            .unwrap();

    println!("{}, {:?}, {:?}", p.pid, p.name, p.desc);
*/
    let products: Vec<Product> = Query::select_all()
        .from_table("public.product")
        .collect(db.as_ref())
        .unwrap();

    for prod in products {
        let name = prod.name.unwrap();
        let desc = match prod.desc {
            Some(desc) => desc,
            None => "".to_string(),
        };  
        println!("{}  {}  {:?}", prod.pid, name, desc);
    }  

    Ok(Response::with((status::Ok, format!("Db is {}", "OK."))))
}





fn main() {
    let db_url: String = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(_) => "postgres://postgres:xyz_123@localhost:5432/test".to_string()
    };

    println!("connecting to postgres: {}", db_url);

    // here intro rustorm pool
    //let ManagedPool::Postgres(inner_pool) = ManagedPool::init(&db_url, 4).unwrap();
    let inner_pool_wr = ManagedPool::init(&db_url, 4).unwrap();
    //let db = pool.connect().unwrap();
    let inner_pool_option = match inner_pool_wr {
        ManagedPool::Postgres(pool) => Some(pool),
        _ => None 
    };
    let inner_pool = inner_pool_option.unwrap();
    
    // router
    let mut router = Router::new();
    router.get("/", index);
    //router.get("json", json_test);

    // mount
    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static", Static::new(Path::new("./static/")));

    // middleware
    // ready to add middleware around mount entity
    let mut middleware = Chain::new(mount);
    // put db connect pool to persistance cache
    middleware.link(PersistRead::<AppDB>::both(inner_pool));
    //middleware.link(PersistRead::<AppDB>::both(db));

    // http server
    let host = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 8080);
    println!("listening on http://{}", host);
    // boot up
    Iron::new(middleware).http(host).unwrap();
}

