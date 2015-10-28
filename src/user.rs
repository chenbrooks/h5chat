
use std::io::Read;
use uuid::Uuid;
use rand::{thread_rng, Rng};
use chrono::offset::utc::UTC;

// Iron crates
use hyper::header::{Headers, SetCookie};
use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;

use cookie::Cookie as CookiePair;
use urlencoded::UrlEncodedBody;
use persistent::Read as PersistRead;
use rustorm::table::IsTable;
use rustorm::query::Query;
use rustorm::query::Equality;
use rustorm::database::DbError;

use crypto::digest::Digest;
use crypto::md5::Md5;

use iron::headers::ContentType;
use rustc_serialize::json;
use jsonway;


use dbdesign::h5manager::H5Manager;

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
    //let params = req.get_ref::<UrlEncodedBody>().unwrap();
    //let email = &params.get("email").unwrap()[0];
    //let password = &params.get("password").unwrap()[0];
    //let nickname = &params.get("nickname").unwrap()[0];
    //println!("{:?}, {:?}, {:?}", email, password, nickname);
 
    let params = req.get_ref::<UrlEncodedBody>().unwrap();
    let email = t_param!(params.get("email"));
    let password = t_param!(params.get("password"));
    let nickname = t_param!(params.get("nickname"));
    println!("{:?}, {:?}, {:?}", email, password, nickname);
    
    
    let mut sh = Md5::new();
    let salt = random_string(5);
    sh.input_str(&(salt.clone() + "_" + password));
    let out_pwd = sh.result_str();
    
    // generate id with uuid;
    let my_uuid = Uuid::new_v4();
    let current_time = UTC::now();
    
    // insert to db
    Query::insert()
        .into_table("public.h5manager")
        .set("manager_id", &my_uuid)
        .set("email", email)
        .set("salt", &salt)
        .set("password", &out_pwd)
        .set("nickname", nickname)
        .set("created_time", &current_time)
        .execute(db.as_ref());
        
    

    Ok(Response::with((status::Ok, format!("Db is {}", "good."))))
    
}



pub fn login(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<PersistRead<AppDB>>().unwrap();
    let db = pool.connect().unwrap();
 
    let params = req.get_ref::<UrlEncodedBody>().unwrap();
    let email = t_param!(params.get("email"));
    let password = t_param!(params.get("password"));
    println!("{:?}, {:?}", email, password);
    
    
    // check if exist this user in db
    let manager_result: Result<H5Manager, DbError> = Query::select()
        .from_table("public.h5manager")
        .filter("email", Equality::EQ, email)
//        .filter("password", Equality::EQ, password)
        .collect_one(db.as_ref());
        
        
        
    println!("{:?}", manager_result);
    // if exist this user
    if let Ok(manager) = manager_result {
        println!("{:?}", manager);
         // generate cookie id with uuid;
        let cookie_ident = Uuid::new_v4().to_simple_string();
        let mut ck = CookiePair::new("h5chat_ci".to_owned(), cookie_ident);
        ck.path = Some("/".to_owned());
        ck.domain = Some(".h5chat.com".to_owned());
        
        let mut response = Response::new();
        response.set_mut(Header(SetCookie(vec![ck])));

        // 200, Set-Cookie, redirect or json
        res_json!(response, true, "login success.")
        
    }
    else {
        println!("error, no this person.");
        //Ok(Response::with((status::Ok, "no this person.")))
        let mut response = Response::new();
        res_json!(response, false, "no this person.")
    }
    
   
}







