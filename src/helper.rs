// Iron crates
use hyper::header::SetCookie;
use iron::prelude::*;
use iron::status;
use iron::modifiers::Header;

use cookie::Cookie as CookiePair;
use urlencoded::UrlEncodedBody;
use persistent::Read as PersistRead;
//use rustorm::table::IsTable;
use rustorm::query::Query;
use rustorm::query::Equality;
use rustorm::database::DbError;
use redis;
use std::ops::Deref;
use redis::ConnectionLike;
use redis::Commands;

use AppRedis;


