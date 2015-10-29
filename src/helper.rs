



pub fn check_cookie_exist(req: &&mut Request, cookie_ident: String) -> bool {
    let pool = req.get::<PersistRead<AppRedis>>().unwrap();
    let conn = pool.get().unwrap();
    
    let cookie_key = "UserCookie:".to_string() + &cookie_ident;
    
    match conn.get(&cookie_key) {
        Ok(_) => true,
        Err(_) => false
    }
} 

pub fn record_user_cookie(req: &&mut Request, cookie_ident: String) {
    let pool = req.get::<PersistRead<AppRedis>>().unwrap();
    let conn = pool.get().unwrap();
    
    let cookie_key = "UserCookie:".to_string() + &cookie_ident;
    
    match conn.set(&cookie_key, "1") {
        Ok(_) => println!("store success"),
        Err(_) => println!("store failed")
    }

    conn.expire(&cookie_key, 3600 * 24 * 3);
    
}
