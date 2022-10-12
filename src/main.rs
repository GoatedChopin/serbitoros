use rocket::{fs::{FileServer, relative}, launch};  //
// mod indexer;

#[launch]
fn rocket() -> _ {
    // indexer::main();
    rocket::build().mount("/", FileServer::from(relative!("/src/static")))
}
