// Copyright 2024 Birnadin Erick
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
struct GCD {
    numbers: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(get_gcd)
            .route("/", web::get().to(get_index))
    });

    println!("Serving on 2003...");
    server
        .bind("127.0.0.1:2003")
        .expect("Failed to bind to interface.")
        .run()
        .await
}

async fn get_index() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
<html>
<head>
 <title>GCD Calculator | by Birnadin E.</title>
</head>
<body>
<p>Input comma separated list of numbers you need to find GCD of </p>
 <form>
 <input type="text" name="numbers" autofocus/>
 <button hx-target=".history" hx-swap="afterbegin" hx-get="/gcd" hx-include="[name='numbers']" type="submit">Compute GCD</button>
 </form>

<ul class="history">
</ul>
<script src="https://unpkg.com/htmx.org@1.9.10" integrity="sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC" crossorigin="anonymous"></script>
</body>
</html>
 "#,
    )
}

#[get("/gcd")]
async fn get_gcd(q: web::Query<GCD>) -> impl Responder {
    // get array of numbers from query string
    let numbers: Vec<&str> = q.numbers.split(",").collect();

    // parse numbers into u64
    let numbers: Vec<u64> = numbers
        .iter()
        .map(|a| u64::from_str(&a).expect("parsing failed"))
        .collect();

    // gcd of given numbe
    let d = match numbers.into_iter().reduce(gcd) {
        Some(x) => x,
        None => 0,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("<li>gcd({}) = {}</li>", q.numbers, d))
}

/// gcd(m, n) utilizes Euclid Algorithm for finding GCD
/// * recursive function to eliminate borrow headache
fn gcd(m: u64, n: u64) -> u64 {
    if n == 0 {
        return m;
    }

    let r = m % n;
    gcd(n, r)
}
