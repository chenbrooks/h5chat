extern crate rustc_serialize;
extern crate hyper;
extern crate redis;

use std::io::Read;
use rustc_serialize::json;
use std::collections::HashMap;

use hyper::Client;
use hyper::header;
use std::thread::sleep_ms;
use redis::Commands;

fn store_to_redis (token: String){
    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1:6379/").unwrap();
    let con = client.get_connection().unwrap();
    // throw away the result, just make sure it does not fail
    let _:() = con.set("gotye_server_token", token).unwrap();
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    let t = con.get("gotye_server_token").unwrap_or("".to_string());
    println!("retreived token is {}", t);
}

#[derive(RustcEncodable, RustcDecodable)]
struct RetValue {
    expires_in: u32,
    access_token: String
}


fn main() {

    // Create a client.
    let client = Client::new();

    let mut json_to_send = HashMap::new();
    json_to_send.insert("username", "daogangtang@126.com");
    json_to_send.insert("grant_type", "password");
    json_to_send.insert("password", "77730150");

    let json_to_send_string = json::encode(&json_to_send).unwrap_or("{}".to_string());


    loop {

        let mut cres = client.post("https://rest.gotye.com.cn/api/accessToken")
            // set a header
            //.header(header::Accept("application/json"))
            .header(header::ContentType::json())
            .body(&json_to_send_string)
            // let 'er go!
            .send().unwrap();

        // 这里处理一下亲加api的返回值
        // TODO:
        let mut body = String::new();
        cres.read_to_string(&mut body).unwrap();
        println!("ret value: {}", body);
        
        let ret_val: RetValue = json::decode(&body).unwrap();
        store_to_redis(ret_val.access_token);

        // 12小时获取一次
        sleep_ms(1000 * 3600 * 12);
    }

}

