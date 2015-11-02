
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

use dbdesign::h5chatroom::H5Chatroom;

use AppDB;
use AppRedis;

use midware::UserCookie;
use midware::ManagerId;



pub fn create(req: &mut Request) -> IronResult<Response> {
    let db = get_db!(req);
    //let manager_id = req.extensions.get::<ManagerId>().unwrap().clone();
    //let manager_id = *req.extensions.get::<ManagerId>().unwrap();
    let manager_id = get_ext_param!(req, ManagerId);
    
    // 获取参数
    //let params = req.get_ref::<UrlEncodedBody>().unwrap();
    let params = get_body_params!(req);
    let name = t_param!(params.get("name"));
    let welcome = t_param!(params.get("welcome"));
    //~ let is_private = t_param_default!(params.get("is_private"), false);
    //~ let is_fakename = t_param_default!(params.get("is_fakename"), false);
    println!("{:?}, {:?}", name, welcome);
    
    
    // generate id with uuid;
    let room_id = Uuid::new_v4();
    
    println!("{:?}, {:?}", room_id, manager_id);
    
    let current_time = UTC::now();
    
    // send a request to gotye to create a chatroom, and get the return json
    // to retreive the ex_room_id
    // Syncronization
    
    // insert to db
    Query::insert()
        .into_table("public.h5chatroom")
        .set("room_id", &room_id)
        .set("manager_id", &manager_id)
        .set("name", name)
        .set("welcome", welcome)
        .set("created_time", &current_time)
        .execute(db.as_ref()).unwrap();
        
    // after creating a new room, return to the main page of board
    res_redirect!("/board/index.html")
    
}


pub fn myrooms(req: &mut Request) -> IronResult<Response> {
    let db = get_db!(req);
    let manager_id = get_ext_param!(req, ManagerId);
    
    // generate id with uuid;
    //let my_uuid = Uuid::parse_str(&manager_id).unwrap();
    
    println!("{:?}", manager_id);
    
        
    let rooms: Vec<H5Chatroom> = Query::select_all()
                             .from_table("public.h5chatroom")
                             .filter("manager_id", Equality::EQ, &manager_id)
                             .collect(db.as_ref())
                             .unwrap();   
    
    //~ for room in rooms {
        //~ let name = prod.name.unwrap();
        //~ let desc = match prod.description {
            //~ Some(desc) => desc,
            //~ None => "".to_string(),
        //~ };
        //~ println!("{}  {}  {:?}", prod.product_id, name, desc);
    //~ }
    //~ let json_reply = jsonway::object(|j| {
            //~ j.set("success", true); 
            //~ j.array("rooms", |arr| {
                //~ for room in rooms {
                    //~ let _room = jsonway::object(|obj| {
                        //~ obj.set("room_id", room.room_id.to_hyphenated_string());
                        //~ obj.set("name", room.name);
                        //~ obj.set("welcome", room.welcome);
                    //~ });
                    
                    //~ arr.push(_room);
                //~ }
            //~ });
    //~ }).unwrap();

    let json_reply = jsonway::object(|j| {
            j.set("success", true); 
            j.set("test", "123".to_owned()); 
            
    }).unwrap();
    
    println!("{:?}", json_reply);
    res_json!(json_reply)

}

