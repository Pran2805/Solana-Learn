use rand::{Rng, rng}; // external library 
// cargo add package_name
// actix fast http server
// tokio asynchronous runtime
// reqwest sends and http request
// sqlx sql database

pub fn random(){
   let mut rng = rng();
   let n:u32 = rng.r#gen(); // gen and r#gen is deprecatd but still works with some warning

   println!("Random Number: {}", n);
}