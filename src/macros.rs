

macro_rules! t_500 {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(e) => {
            println!("Errored: {:?}", e); 
            return Ok(Response::with((status::InternalServerError)))
        }   
    })  
}

// check the validation of parameters, if not valid, return BadRequest to 
macro_rules! t_param {
    ($expr:expr) => (match $expr {
        Some(val) => &val[0],
        None => {
            println!("missing param");; 
            return Ok(Response::with((status::BadRequest, "missing parameter.")))
        }   
    })  
}

macro_rules! t_param_default {
    ($expr:expr, $default:expr) => (match $expr {
        Some(val) => &val[0],
        None => {
            println!("missing param");; 
            $default
        }   
    })  
}


macro_rules! res_json_success2 {

    ($response:expr, $succ:expr, $info_key:expr, $info:expr) => ({
        
        let json_reply = jsonway::object(|j| {
            j.set("success", $succ); 
            j.set(&$info_key.to_owned(), $info.to_owned());
        }).unwrap();
        
        $response.set_mut(status::Ok);
        $response.set_mut(Header(ContentType::json()));
        $response.set_mut(json::encode(&json_reply).unwrap());
    
        Ok($response)
    })  
}

macro_rules! res_json_success {

    ($succ:expr, $info_key:expr, $info:expr) => ({
        let mut response = Response::new();
        
        let json_reply = jsonway::object(|j| {
            j.set("success", $succ); 
            j.set(&$info_key.to_owned(), $info.to_owned());
        }).unwrap();
        
        response.set_mut(status::Ok);
        response.set_mut(Header(ContentType::json()));
        response.set_mut(json::encode(&json_reply).unwrap());
    
        Ok(response)
    })  
}

macro_rules! res_json2 {

    ($res:expr, $json_data:expr) => ({
        $res.set_mut(status::Ok);
        $res.set_mut(Header(ContentType::json()));
        $res.set_mut(json::encode(&$json_data).unwrap());
    
        Ok($res)
    })  
}


macro_rules! res_json {

    ($json_data:expr) => ({
        let mut response = Response::new();
        response.set_mut(status::Ok);
        response.set_mut(Header(ContentType::json()));
        response.set_mut(json::encode(&$json_data).unwrap());
    
        Ok(response)
    })  
}


macro_rules! get_db {
    ($req:expr) => ({
        let pool = $req.get::<PersistRead<AppDB>>().unwrap();
        let db = pool.connect().unwrap();
        db
    })  
}

macro_rules! get_redis {
    ($req:expr) => ({
        let redis_pool = $req.get::<PersistRead<AppRedis>>().unwrap();
        let redis_client_wr = redis_pool.get().unwrap();
        let redis_client = redis_client_wr.deref();
        let conn = redis_client.get_connection().unwrap();
        conn
    })  
}
    
macro_rules! get_ext_param {
    ($req:expr, $keyname:ident) => ({
        $req.extensions.get::<$keyname>().unwrap().clone()
    })  
}

macro_rules! get_query_params {
    ($req:expr) => ({
        $req.get_ref::<UrlEncodedQuery>().unwrap()
    })  
}

macro_rules! get_body_params {
    ($req:expr) => ({
        $req.get_ref::<UrlEncodedBody>().unwrap()
    })  
}

macro_rules! res_redirect {
    ($path:expr) => ({
        let urlstr = "http://127.0.0.1:8080".to_owned() + $path;
        let mut response = Response::new();
        let url = Url::parse(&urlstr).unwrap();
        response.set_mut(status::Found).set_mut(Redirect(url));
        Ok(response)
    })  
    
    
}
    
