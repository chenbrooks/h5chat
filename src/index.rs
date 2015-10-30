
// Iron crates
use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;
use iron::Url;


pub fn index(req: &mut Request) -> IronResult<Response> {
    let url = Url::parse("http://127.0.0.1:8080/page/index.html").unwrap();
    Ok(Response::with((status::Found, Redirect(url) )))
}




