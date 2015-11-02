// 可以使用这个工具检查配置有没有错误

use toml;
use toml::{Table};
use std::fs::File;
use std::io::{Read};
use std::cell::RefCell;


lazy_static! {
    static ref global_config: Table = get_config();
}


// 这个的好处在于可以实时更新，不需要重启服务
pub fn get_config () -> Table {
    let mut input = String::new();
    File::open("./myconfig.toml").and_then(|mut f| {
        f.read_to_string(&mut input)
    }).unwrap();

    let config = toml::Parser::new(&input).parse().unwrap();
    println!("toml parse success.");
    config
}

// 返回一个聊天室的配置
pub fn get_room_config (id: i64) -> (i64, i64, String, String, String, String, String, String){
    //println!("Get room info: {}", id);
    let config = &global_config;
    let rooms = config.get("rooms").unwrap();
    
    let query_id = format!("room_{}.id", id);
    let room_id = rooms.lookup(&query_id).unwrap().as_integer().unwrap();
    
    let query_group = format!("room_{}.bind_group", id);
    let bind_group = rooms.lookup(&query_group).unwrap().as_integer().unwrap();
    
    let query_name = format!("room_{}.name", id);
    let room_name = rooms.lookup(&query_name).unwrap().as_str().unwrap();
    
    let query_title = format!("room_{}.title", id);
    let room_title = rooms.lookup(&query_title).unwrap().as_str().unwrap();
    
    let query_logo = format!("room_{}.logo", id);
    let room_logo = rooms.lookup(&query_logo).unwrap().as_str().unwrap();
    
    let query_url = format!("room_{}.wx_url", id);
    let wx_url = rooms.lookup(&query_url).unwrap().as_str().unwrap();
    
    let query_welcome = format!("room_{}.welcome", id);
    let room_welcome = rooms.lookup(&query_welcome).unwrap().as_str().unwrap();
    
    let query_brand = format!("room_{}.brand_title", id);
    let brand_title = rooms.lookup(&query_brand).unwrap().as_str().unwrap();
    
    (   
        room_id, 
        bind_group,
        room_name.to_string(),

        room_title.to_string(),
        room_logo.to_string(),
        wx_url.to_string(), 
        room_welcome.to_string(),
	brand_title.to_string()
    )
}

pub fn get_room_str_attr (id: i64, attr: String) -> String {
    let config = &global_config;
    let rooms = config.get("rooms").unwrap();
    
    let query = format!("room_{}.", id) + &attr;
    let attr_ret = rooms.lookup(&query).unwrap().as_str().unwrap();

    attr_ret.to_string()
}

pub fn get_room_int_attr (id: i64, attr: String) -> i64 {
    let config = &global_config;
    let rooms = config.get("rooms").unwrap();
    
    let query = format!("room_{}.", id) + &attr;
    let attr_ret = rooms.lookup(&query).unwrap().as_integer().unwrap();

    attr_ret
}

// 返回一个专家群组的配置
pub fn get_group_config (id: i64) -> (i64, String, Vec<i64>, String, String) {
    
    let config = &global_config;
    let groups = config.get("groups").unwrap();
    
    let query_id = format!("group_{}.id", id);
    let group_id = groups.lookup(&query_id).unwrap().as_integer().unwrap();
    
    let query_name = format!("group_{}.name", id);
    let group_name = groups.lookup(&query_name).unwrap().as_str().unwrap();
    
    let query_rooms = format!("group_{}.bind_rooms", id);
    let bind_rooms = groups.lookup(&query_rooms).unwrap().as_slice().unwrap();
    
    let query_org = format!("group_{}.org", id);
    let group_org = groups.lookup(&query_org).unwrap().as_str().unwrap();
    
    let query_title = format!("group_{}.org_title", id);
    let org_title = groups.lookup(&query_title).unwrap().as_str().unwrap();
    
    // 将数值转化为可以
    let mut vecret: Vec<i64> = Vec::new();
    for room in bind_rooms {
        //println!("{}", room);
        vecret.push(room.as_integer().unwrap());
    }
                        
    (
        group_id, 
        group_name.to_string(), 
        vecret,
        group_org.to_string(),
        org_title.to_string()
    )
    
}

pub fn get_group_str_attr (id: i64, attr: String) -> String {
    let config = &global_config;
    let rooms = config.get("groups").unwrap();
    
    let query = format!("group_{}.", id) + &attr;
    let attr_ret = rooms.lookup(&query).unwrap().as_str().unwrap();

    attr_ret.to_string()
}

pub fn get_group_int_attr (id: i64, attr: String) -> i64 {
    let config = &global_config;
    let rooms = config.get("groups").unwrap();
    
    let query = format!("group_{}.", id) + &attr;
    let attr_ret = rooms.lookup(&query).unwrap().as_integer().unwrap();

    attr_ret
}

pub fn get_group_str_array (id: i64, attr: String) -> Vec<String> {
    let config = &global_config;
    let rooms = config.get("groups").unwrap();
    
    let query = format!("group_{}.", id) + &attr;
    let str_list = rooms.lookup(&query).unwrap().as_slice().unwrap();

    let mut vecret: Vec<String> = Vec::new();
    for str in str_list {
        //println!("{}", room);
        vecret.push(str.as_str().unwrap().to_string());
    }
    
    vecret
}

// 返回一个聊天室的配置
pub fn get_default_room_list_config () -> Vec<i64> {
    //println!("Get room info: {}", id);
    let config = &global_config;
    let rooms = config.get("users").unwrap();
    
    let query = "org_default.bind_rooms";
    let room_list = rooms.lookup(&query).unwrap().as_slice().unwrap();
    
        
    // 将数值转化为可以
    let mut vecret: Vec<i64> = Vec::new();
    for room in room_list {
        //println!("{}", room);
        vecret.push(room.as_integer().unwrap());
    }
    
    vecret
}

pub fn get_room_list_config_of_group (group_id: i64) {

    
    let (group_id, group_name, rooms, org, org_title) = get_group_config(group_id);
    println!("\n---------------------------------------------------");
    println!("GROUP: {}, {}, {:?}, {}, {}", group_id, group_name, rooms, org, org_title);
    for room in rooms {
        println!("room: {}", room);
        let (room_id, bind_group, room_name, title, logo, wx_url, welcome, _) = get_room_config(room);
        println!("room content: {}, {}, {}, {}, {}, {}", room_id, room_name, wx_url, bind_group, title, welcome);
        
        assert!(group_id == bind_group, 
            "组与聊天室关系不匹配 group: {}, room: {}", group_id, room_id);
        
    }
    println!("^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n");
    
}

pub fn get_room_list_of_group (group_id: i64) -> Vec<(i64, String)>{

    let mut vec = Vec::new();
    let (group_id, group_name, rooms, org, org_title) = get_group_config(group_id);
    for room in rooms {
        let (room_id, bind_group, room_name, title, logo, wx_url, welcome, _) = get_room_config(room);
        vec.push((room_id, room_name));
    }
    
    vec
}


pub fn get_room_list (id_list: Vec<i64>) -> Vec<(i64, String)>{

    let mut vec = Vec::new();
    for room in id_list {
        //println!("room: {}", room);
        let (room_id, bind_group, room_name, title, logo, wx_url, welcome, _) = get_room_config(room);
        //println!("room content: {}, {}, {}, {}, {}, {}", room_id, room_name, wx_url, bind_group, title, welcome);
        vec.push((room_id, room_name));
    }
    
    vec
}













