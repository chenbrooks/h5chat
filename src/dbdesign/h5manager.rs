
use uuid::Uuid;
use chrono::datetime::DateTime;
use chrono::offset::utc::UTC;

use rustorm::dao::{Dao, IsDao};

use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;





///
/// This will be exposed as an @Api, including @Table(users, category, product_availability, photo)
///
#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct H5Manager {
    pub manager_id:     Uuid,
    pub email:          String,
    pub name:           Option<String>,
    pub nickname:       String,
    pub password:       String,
    pub salt:           String,
    pub created_time:   DateTime<UTC>,
    
    pub company:        Option<String>,
    pub phone:          Option<String>,
    pub im:             Option<String>,
    pub logo:           Option<String>,
    pub extra:          Option<String>
}


impl IsDao for H5Manager{
    fn from_dao(dao: &Dao) -> Self {
        H5Manager {
            manager_id: dao.get("manager_id"),
            email: dao.get("email"),
            nickname: dao.get("nickname"),
            password: dao.get("password"),
            salt: dao.get("salt"),
            created_time: dao.get("created_time"),
            name: dao.get_opt("name"),
            
            company: dao.get_opt("company"),
            phone: dao.get_opt("phone"),
            im: dao.get_opt("im"),
            logo: dao.get_opt("logo"),
            extra: dao.get_opt("extra"),
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        
        dao.set("manager_id", &self.manager_id);
        dao.set("email", &self.email);
        dao.set("nickname", &self.nickname);
        dao.set("password", &self.password);
        dao.set("salt", &self.salt);
        
        match self.name {
            Some(ref name) => dao.set("name", name),
            None => dao.set_null("name"),
        }
        match self.company {
            Some(ref company) => dao.set("company", company),
            None => dao.set_null("company"),
        }
        match self.phone {
            Some(ref phone) => dao.set("phone", phone),
            None => dao.set_null("phone"),
        }
        match self.im {
            Some(ref im) => dao.set("im", im),
            None => dao.set_null("im"),
        }
        match self.logo {
            Some(ref logo) => dao.set("logo", logo),
            None => dao.set_null("logo"),
        }
        match self.extra {
            Some(ref extra) => dao.set("extra", extra),
            None => dao.set_null("extra"),
        }
        
        dao
    }
}

impl IsTable for H5Manager {

    fn table() -> Table {

        Table{
            schema: "publick".to_string(),
            name:   "h5manager".to_string(),
            parent_table:   None,
            sub_table:      vec![],
            comment:        None,
            columns:        vec![
                Column {
                    name:           "manager_id".to_string(),
                    data_type:      "Uuid".to_string(),
                    db_data_type:   "uuid".to_string(),
                    is_primary:     true, 
                    is_unique:      true, 
                    not_null:       true, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "email".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      true, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "nickname".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "password".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       true, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "salt".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "created_time".to_string(),
                    data_type:      "DateTime<UTC>".to_string(),
                    db_data_type:   "timestamp without time zone".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       true, 
                    is_inherited:   false,
                    default:        Some("now()".to_string()),
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "company".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "phone".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "im".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "logo".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                Column {
                    name:           "extra".to_string(),
                    data_type:      "String".to_string(),
                    db_data_type:   "character varying".to_string(),
                    is_primary:     false, 
                    is_unique:      false, 
                    not_null:       false, 
                    is_inherited:   false,
                    default:        None,
                    comment:        None,
                    foreign:        None,
                },
                
            ],
            is_view: false
        }
    }
}
