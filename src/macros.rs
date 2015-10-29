

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


macro_rules! res_json {

    ($response:expr, $succ:expr, $info:expr) => ({
        
        let json_reply = jsonway::object(|j| {
            j.set("success", $succ); 
            j.set("info", $info.to_owned());
        }).unwrap();
        
        $response.set_mut(status::Ok);
        $response.set_mut(Header(ContentType::json()));
        $response.set_mut(json::encode(&json_reply).unwrap());
    
        Ok($response)
    })  
}

