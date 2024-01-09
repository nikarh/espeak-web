use std::{env, net::Ipv4Addr, process, sync::Mutex};

fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8585);

    eprintln!("Starting server on port {port}");

    let guard = Mutex::<()>::default();

    let Ok(server) = tiny_http::Server::http((Ipv4Addr::UNSPECIFIED, port)) else {
        eprintln!("Failed to start server");
        return;
    };

    for mut request in server.incoming_requests() {
        if request.method() != &tiny_http::Method::Post {
            let _ = request.respond(tiny_http::Response::empty(404));
            continue;
        }

        let mut body = String::new();
        if let Err(err) = request.as_reader().read_to_string(&mut body) {
            eprintln!("Failed to read request body: {err}");
            let _ = request.respond(tiny_http::Response::empty(400));
            continue;
        }

        eprintln!("Speaking `{body}`");

        // Espeak can not parallelize
        let _lock = guard.lock().unwrap();
        if let Err(err) = process::Command::new("espeak").arg(body).output() {
            eprintln!("Espeak failed: {err}");
        }

        let _ = request.respond(tiny_http::Response::empty(204));
    }
}
