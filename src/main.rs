use tiny_http::{Server, Response, Request};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

fn handle_request(req: Request, kv: Arc<Mutex<HashMap<String, String>>>) {
    let url = req.url();
    let response = if url.starts_with("/put") {
        let query = url.split('?').nth(1).unwrap_or("");
        let pairs: Vec<(&str, &str)> = query.split('&')
            .filter_map(|kv| {
                let mut parts = kv.split('=');
                Some((parts.next()?, parts.next()?))
            }).collect();

        let mut map = kv.lock().unwrap();
        let mut out = String::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v.to_string());
            out += &format!("Stored: {} = {}
", k, v);
        }
        Response::from_string(out)
    } else if url.starts_with("/get") {
        let key = url.split('=').nth(1).unwrap_or("");
        let map = kv.lock().unwrap();
        let value = map.get(key).cloned().unwrap_or("Not found".to_string());
        Response::from_string(value)
    } else {
        Response::from_string("Use /put?key=foo&value=bar or /get?key=foo")
    };

    let _ = req.respond(response);
}

fn main() {
    let kv_store = Arc::new(Mutex::new(HashMap::new()));
    let server = Server::http("0.0.0.0:8080").unwrap();

    for request in server.incoming_requests() {
        let kv = Arc::clone(&kv_store);
        std::thread::spawn(move || handle_request(request, kv));
    }
}
