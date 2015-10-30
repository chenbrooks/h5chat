
use std::io::Read;
use std::ops::Deref;
use uuid::Uuid;
use rand::{thread_rng, Rng};
use chrono::offset::utc::UTC;

// Iron crates
use hyper::header::SetCookie;
use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;

use cookie::Cookie as CookiePair;
use urlencoded::UrlEncodedBody;
use persistent::Read as PersistRead;
//use rustorm::table::IsTable;
use rustorm::query::Query;
use rustorm::query::Equality;
use rustorm::database::DbError;

use crypto::digest::Digest;
use crypto::md5::Md5;

use iron::headers::ContentType;
use rustc_serialize::json;
use jsonway;

use redis::Commands;

use iron::modifiers::Redirect;
use iron::Url;

use dbdesign::h5manager::H5Manager;

use AppDB;
use AppRedis;

use midware::UserCookie;


fn random_string(length: usize) -> String {
    thread_rng().gen_ascii_chars().take(length).collect()
}


//~ pub fn register_view(req: &mut Request) -> IronResult<Response> {


    //~ let mut data: HashMap<String, String> = HashMap::new();
    //~ let mut resp = Response::new();
    //~ resp.set_mut(Template::new("register", data)).set_mut(status::Ok);
    //~ Ok(resp)
//~ }

pub fn register(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<PersistRead<AppDB>>().unwrap();
    let db = pool.connect().unwrap();

    let params = req.get_ref::<UrlEncodedBody>().unwrap();
    let email = t_param!(params.get("email"));
    let password = t_param!(params.get("password"));
    let nickname = t_param!(params.get("nickname"));
    println!("{:?}, {:?}, {:?}", email, password, nickname);
    
    
    let mut sh = Md5::new();
    let salt = random_string(6);
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
        .execute(db.as_ref()).unwrap();
        
    

    Ok(Response::with((status::Ok, format!("Db is {}", "good."))))
    
}

//~ pub fn login_view(req: &mut Request) -> IronResult<Response> {


    //~ let mut data: HashMap<String, String> = HashMap::new();
    //~ let mut resp = Response::new();
    //~ resp.set_mut(Template::new("login", data)).set_mut(status::Ok);
    //~ Ok(resp)
//~ }

pub fn login(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<PersistRead<AppDB>>().unwrap();
    let db = pool.connect().unwrap();
    
    let redis_pool = req.get::<PersistRead<AppRedis>>().unwrap();
    let redis_client_wr = redis_pool.get().unwrap();
    let redis_client = redis_client_wr.deref();
    let conn = redis_client.get_connection().unwrap();
        
 
    let params = req.get_ref::<UrlEncodedBody>().unwrap();
    let email = t_param!(params.get("email"));
    let password = t_param!(params.get("password"));
    println!("{:?}, {:?}", email, password);
    
    // check if exist this user in db
    let manager_result: Result<H5Manager, DbError> = 
        Query::select_all()
        .from_table("public.h5manager")
        .filter("email", Equality::EQ, email)
        .collect_one(db.as_ref());
        
    println!("{:?}", manager_result);
    // if exist this user
    if let Ok(manager) = manager_result {
        
        let str2test = manager.salt.to_owned() + "_" + password;
        let mut sh = Md5::new();
        sh.input_str(&str2test);
        let out_str = sh.result_str();
        if out_str == manager.password {
             // generate cookie id with uuid;
            let cookie_ident = Uuid::new_v4().to_simple_string();
            let mut ck = CookiePair::new("h5chat_ci".to_owned(), cookie_ident.clone());
            ck.path = Some("/".to_owned());
            //ck.domain = Some(".h5chat.com".to_owned());
            ck.max_age = Some(3600 * 24 * 3);   // three days
            
            let cookie_key = "UserCookie:".to_string() + &cookie_ident;
            let _: () = conn.set(&cookie_key[..], "1").unwrap();
            let _: () = conn.expire(&cookie_key[..], 3600 * 24 * 3).unwrap();
            
            let mut response = Response::new();
            response.set_mut(Header(SetCookie(vec![ck])));

            // 200, Set-Cookie, redirect or json
            res_json!(response, true, "login success.")
        }
        else {
            let mut response = Response::new();
            res_json!(response, false, "email or password not correct.")
        }
        
    }
    else {
        println!("error, no this person.");
        //Ok(Response::with((status::Ok, "no this person.")))
        let mut response = Response::new();
        res_json!(response, false, "no this person.")
    }
    
   
}


pub fn logout(req: &mut Request) -> IronResult<Response> {
    let redis_pool = req.get::<PersistRead<AppRedis>>().unwrap();
    let redis_client_wr = redis_pool.get().unwrap();
    let redis_client = redis_client_wr.deref();
    let conn = redis_client.get_connection().unwrap();
        
    // get user's cookie
    let cookie_ident = req.extensions.get::<UserCookie>().unwrap();
    let cookie_key = "UserCookie:".to_string() + &cookie_ident;
    let _: () = conn.del(&cookie_key[..]).unwrap();
    
    let mut response = Response::new();

    let url = Url::parse("http://127.0.0.1:8080/page/index.html").unwrap();
    response.set_mut(status::Found).set_mut(Redirect(url));
    Ok(response)
}





