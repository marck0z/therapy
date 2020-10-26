use actix_web::{
    App, HttpResponse, HttpServer, web,
};
use serde::{Deserialize, Serialize};

use expression::*;

mod expression;

/// To be able to add, remove and override substitutions dynamically
/// a small expression processor was created
///
/// To declare a substitution we use the same format as in the assignment
///
/// Two types of substitutions are recognized:
/// those who change the value of H and are affected by the value of A, B and C
/// and those who change the value of K and are affected by the values of H, D, E and F
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //H substitution strings
    let mut h_subs_str = Vec::new();
    //K substitution strings
    let mut k_subs_str = Vec::new();

    //Base
    h_subs_str.push("A && B && !C => H = M");
    h_subs_str.push("A && B && C => H = P");
    h_subs_str.push("!A && B && C => H = T");
    k_subs_str.push("H = M => K = D + (D * E / 10)");
    k_subs_str.push("H = P => K = D + (D * (E - F) / 25.5)");
    k_subs_str.push("H = T => K = D - (D * F / 30)");

    //Custom 1
    k_subs_str.push("H = P => K = 2 * D + (D * E / 100)");

    //Custom 2
    h_subs_str.push("A && B && !C => H = T");
    h_subs_str.push("A && !B && C => H = M");
    k_subs_str.push("H = M => K = F + D + (D * E / 100)");

    //create an expression processor from the substitution strings provided
    //this will be used to evaluate input from the user and calculate H and K
    let ep = ExpressionProcessor::new(h_subs_str, k_subs_str);

    //Create the HTTP Server
    HttpServer::new(move || App::new()
        .data(ep.clone())
        .service(web::resource("/eval").route(web::post().to(eval))))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/// REST API endpoint
async fn eval(input: web::Json<Input>, ep: web::Data<ExpressionProcessor>) -> HttpResponse {
    println!("{:?}", input);
    if let Ok(r) = ep.evaluate(&*input) {
        HttpResponse::Ok().json(r)
    } else {
        HttpResponse::BadRequest().finish()
    }
}

/// REST API Input
#[derive(Deserialize, Debug)]
struct Input {
    #[serde(rename = "A")]
    a: bool,
    #[serde(rename = "B")]
    b: bool,
    #[serde(rename = "C")]
    c: bool,
    #[serde(rename = "D")]
    d: f64,
    #[serde(rename = "E")]
    e: i64,
    #[serde(rename = "F")]
    f: i64,
}

/// REST API Output
#[derive(Serialize, Debug, PartialEq)]
struct Output {
    #[serde(rename = "H")]
    h: char,
    #[serde(rename = "K")]
    k: f64,
}

