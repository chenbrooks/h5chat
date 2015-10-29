
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::request::Request;
use iron::IronResult;
use iron::headers::Cookie;
use cookie::Cookie as CookieObj;

pub struct CheckLogin;

impl typemap::Key for CheckLogin { type Value = bool; }

impl BeforeMiddleware for CheckLogin {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        
        // get the cookie part in the request headers
        let cookiep =  req.headers.get::<Cookie>();
        println!("{:?}", cookiep);
        match cookiep {
            Some(ref value) => {
                //println!("{:?}", value);
                let Cookie(ref ckvec) = **value;
                //println!("{:?}", ckvec);
                let cookie_vec = ckvec.iter()
                                    .filter(|item: &&CookieObj| item.name == "h5chat_ci".to_owned())
                                    .take(1)
                                    .collect::<Vec<&CookieObj>>();
                //println!("{:?}", cookie_vec);
                let cookie_obj = cookie_vec[0];
                //println!("{:?}", cookie_obj);
                let cookie_value = cookie_obj.value.clone();
                //println!("{:?}", cookie_value);
                
                // pass this cookie_value to redis 
                let exist = check_cookie_exist(&req, cookie_value);
                if exist {
                    req.extensions.insert::<CheckLogin>(true);
                }
                else {
                    req.extensions.insert::<CheckLogin>(false);
                }
            },
            None => {
                println!("no cookie");
                req.extensions.insert::<CheckLogin>(false);
            }
        }
        
        Ok(())
    }
}
