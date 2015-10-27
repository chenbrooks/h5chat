

use rustorm::table::IsTable;
use rustorm::query::Query;
use uuid::Uuid;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::io::Read;
use rand::{thread_rng, Rng};

// Iron crates
use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;
use persistent::Read as PersistRead;


use AppDB;


fn random_string(length: usize) -> String {
    thread_rng().gen_ascii_chars().take(length).collect()
}


pub fn register(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<PersistRead<AppDB>>().unwrap();
    let db = pool.connect().unwrap();

    // get the post body
    //let mut payload = String::new();
    //t500!(req.body.read_to_string(&mut payload));
    
    //let name = params.get("name").unwrap();
    //let password = params.get("password").unwrap();
    //let nickname = params.get("nickname").unwrap();
    //println!("{:?}, {:?}, {:?}", name[0], password[0], nickname[0]);
    
    // parse params
    let params = req.get_ref::<UrlEncodedBody>().unwrap();
    let email = &params.get("email").unwrap()[0];
    let password = &params.get("password").unwrap()[0];
    let nickname = &params.get("nickname").unwrap()[0];
    println!("{:?}, {:?}, {:?}", email, password, nickname);
    
    // TODO: we should validate these parameters.
    
    
    let mut sh = Md5::new();
    let salt = random_string(5);
    sh.input_str(&(salt.clone() + "_" + password));
    let out_pwd = sh.result_str();
    
    // generate id with uuid;
    let my_uuid = Uuid::new_v4();
    
    // insert to db
    Query::insert()
        .into_table("public.h5manager")
        .set("manager_id", &my_uuid)
        .set("email", email)
        .set("salt", &salt)
        .set("password", &out_pwd)
        .set("nickname", nickname)
        .execute(db.as_ref());
        
    

    Ok(Response::with((status::Ok, format!("Db is {}", "good."))))
}







