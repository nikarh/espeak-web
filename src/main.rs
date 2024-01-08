use std::{env, net::Ipv4Addr, process, sync::Mutex};

use rouille::{try_or_400, Response};

fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8585);

    eprintln!("Starting server on port {port}");

    let guard = Mutex::<()>::default();
    rouille::start_server((Ipv4Addr::UNSPECIFIED, port), {
        move |request| {
            if request.method() != "POST" {
                return Response::empty_404();
            }

            let body = try_or_400!(rouille::input::plain_text_body(request));
            eprintln!("Speaking `{body}`");

            // Espeak can not parallelize
            let _lock = guard.lock().unwrap();
            if let Err(err) = process::Command::new("espeak").arg(body).output() {
                eprintln!("Espeak failed: {err}");
            }

            Response::empty_204()
        }
    });
}
