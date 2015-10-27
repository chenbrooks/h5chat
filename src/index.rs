
use rustorm::dao::{Dao, IsDao};
use rustorm::table::Table;
use rustorm::table::IsTable;
use rustorm::query::Query;


// Iron crates
use iron::prelude::*;
use iron::status;
use persistent::Read as PersistRead;


// define this to use it with iron persistance cache plugin
use AppDB;


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

pub fn index(req: &mut Request) -> IronResult<Response> {
    let pool = req.get::<PersistRead<AppDB>>().unwrap();
    let db = pool.connect().unwrap();
    //let db = t500!(pool.connect());

    let p: Product = Query::select_all()
                            .from_table("public.product")
                            .collect_one(db.as_ref())
                            .unwrap();

    println!("{}, {:?}, {:?}", p.pid, p.name, p.desc);

    Ok(Response::with((status::Ok, format!("Db is {}", "OK."))))
}




