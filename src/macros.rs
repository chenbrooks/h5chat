

macro_rules! t500 {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(e) => {
            println!("Errored: {:?}", e); 
            return Ok(Response::with((status::InternalServerError)))
        }   
    })  
}

