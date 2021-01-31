#[macro_use]
extern crate actix_web; 

use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize; 
use std::cell::Cell; 
use std::sync::atomic::{AtomicUsize, Ordering}; // 
use std::sync::{Arc, Mutex}; // share and mutate things not atomic across multiple threads

// handler that looks for header in get request & responds
// with message based on that header
static SERVER_COUNTER:Atomic Usize = AtomicUsize::new(0);

struct AppState { // constructed in application factory
    server_id: usize, 
    request_count: Cell<usize>,
    messages: Arc<Mutex<Vec<String>>>,
}
//information about state
#[derive(Serialize)]
struct IndexResponse{
    server_id: usize, 
    request_count: usize, 
    messages: Vec<String>,
}

#[get("/")] // indext handler
fn index(state: web::Data<AppState>) -> Result<web::Json<IndexResponse>\
> {
    let request_count = state.request_count.get() + 1; 
    state.request_count.set(request_count);
    let ms  state.messages.lock().unwrap(); 

    Ok(web::Json(IndexResponse{
        server_id: state.server_id, 
        request_count, 
        messages: ms.clone()
    }))

}

pub struct MessageApp{
    port: u16, 
}

impl MessageApp{
    pub fn new(port:u16) -> Self {
        MessageApp{port}
    }

    pub fn run(&self) -> std::io::Result<()> {
        println!("Starting HTTP Server: 127.0.0.1: {}", self.port);
        HttpServer::new(move || {
            App::new()

            .wrap(middleware::Logger::default())
                            .service(index)
        })
        .bind(("127.0.0.1", self.port))?
        .workers(8)
        .run()
    }
}

