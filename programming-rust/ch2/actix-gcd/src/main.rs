use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[tokio::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Starting server on http://localhost:8000");
    server
        .bind("127.0.0.1:8000")
        .expect("error binding server")
        .run()
        .await
        .expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
            <doctype html>
            <html>
                <head>
                    <title>Actix web</title>
                </head>
                <body>
                    <h1>GCD Calculator</h1>
                    <form action="/gcd" method="post">
                        <input type="text" name="n"/>
                        <input type="text" name="m"/>
                        <button type="submit">Compute GCD</button>
                    </form>
                </body>
            </html>
           "#,
    )
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    println!("{:?}", form);

    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }

    let response = format!(
        "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n",
        form.n,
        form.m,
        gcd_recursion(form.n, form.m)
    );

    HttpResponse::Ok().content_type("text/html").body(response)
}

pub fn gcd_recursion(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd_recursion(b, a % b)
    }
}
