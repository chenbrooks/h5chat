extern crate rustorm;
extern crate dbdesign;


use std::env;
use rustorm::pool::ManagedPool;
use rustorm::table::IsTable;
use dbdesign::h5manager::H5Manager;


fn main() {
    let url: String = env::var("H5CHAT_DATABASE_URL").unwrap();
    //let url = "sqlite:///file1.db".to_string();
    println!("db_url is {}", url);
    
    let pool = ManagedPool::init(&url, 1).unwrap();
    let db = pool.connect().unwrap();
    
    //db.as_ddl().create_table(&H5Manager::table());
}

