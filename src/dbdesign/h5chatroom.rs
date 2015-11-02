
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

use rustorm::dao::{Dao, IsDao};

use rustorm::table::IsTable;
//use rustorm::table::Column;
use rustorm::table::Table;




#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct H5Chatroom {
    pub room_id:        Uuid,
    pub manager_id:     Uuid,
    pub name:           String,
    pub welcome:        String,
    pub created_time:   DateTime<UTC>,
    
    pub is_private:     Option<bool>,
    // 允许成员自己修改昵称
    pub is_fakename:    Option<bool>,
    pub ex_room_id:     Option<String>,
    pub ex_group_id:    Option<String>,
    // 成员的默认头衔，在匿名的时候显示，后面会自动接编号
    pub member_title:   Option<String>,
    pub logo:           Option<String>,
    pub history_len:    Option<u64>,
    pub extern_url:     Option<String>,
    pub extra:          Option<String>
}


impl IsDao for H5Chatroom {
    fn from_dao(dao: &Dao) -> Self {
        H5Chatroom {
            room_id: dao.get("room_id"),
            manager_id: dao.get("manager_id"),
            name: dao.get("name"),
            welcome: dao.get("welcome"),
            created_time: dao.get("created_time"),
            
            is_private: dao.get_opt("is_private"),
            is_fakename: dao.get_opt("is_fakename"),
            ex_room_id: dao.get_opt("ex_room_id"),
            ex_group_id: dao.get_opt("ex_group_id"),
            member_title: dao.get_opt("member_title"),
            logo: dao.get_opt("logo"),
            history_len: dao.get_opt("history_len"),
            extern_url: dao.get_opt("extern_url"),
            extra: dao.get_opt("extra"),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        
        dao.set("room_id", &self.room_id);
        dao.set("manager_id", &self.manager_id);
        dao.set("name", &self.name);
        dao.set("welcome", &self.welcome);
        dao.set("created_time", &self.created_time);
        
        match self.is_private {
            Some(ref is_private) => dao.set("is_private", is_private),
            None => dao.set_null("is_private"),
        }
        match self.is_fakename {
            Some(ref is_fakename) => dao.set("is_fakename", is_fakename),
            None => dao.set_null("is_fakename"),
        }
        match self.ex_room_id {
            Some(ref ex_room_id) => dao.set("ex_room_id", ex_room_id),
            None => dao.set_null("ex_room_id"),
        }
        match self.ex_group_id {
            Some(ref ex_group_id) => dao.set("ex_group_id", ex_group_id),
            None => dao.set_null("ex_group_id"),
        }
        match self.member_title {
            Some(ref member_title) => dao.set("member_title", member_title),
            None => dao.set_null("member_title"),
        }
        match self.logo {
            Some(ref logo) => dao.set("logo", logo),
            None => dao.set_null("logo"),
        }
        match self.history_len {
            Some(ref history_len) => dao.set("history_len", history_len),
            None => dao.set_null("history_len"),
        }        
        match self.extern_url {
            Some(ref extern_url) => dao.set("extern_url", extern_url),
            None => dao.set_null("extern_url"),
        }
        match self.extra {
            Some(ref extra) => dao.set("extra", extra),
            None => dao.set_null("extra"),
        }
        
        dao
    }
}

impl IsTable for H5Chatroom {

    fn table() -> Table {

        Table{
            schema: "public".to_string(),
            name:   "h5chatroom".to_string(),
            parent_table:   None,
            sub_table:      vec![],
            comment:        None,
            columns:        vec![],
            is_view: false
        }
    }
}
