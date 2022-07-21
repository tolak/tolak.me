use serde_json::json;
use serde::Serialize;
use worker::*;
use handlebars::Handlebars;

mod utils;

#[derive(Serialize)]
struct Resume {
    name: &'static str,
    email: &'static str,
    ens: &'static str,
    twitter: &'static str,
    github: &'static str,
}

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::new();

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get_async("/", |_req, ctx| async move {
            println!("Fetch templates file from KV");
            let kv = ctx.kv("TOLAK_KV")?;
            match kv.get("templates/index.hbs").text().await? {
                Some(templates) => Response::ok(generate_index_page(templates.as_str())),
                None => Response::error("Bad Request", 400)
            }
        })
        .run(req, env)
        .await
}

#[no_mangle]
pub fn generate_index_page(templates: &str) -> String {
    println!("Rending index page");
    let mut reg = Handlebars::new();
    reg.register_template_string("index", templates).unwrap();
    reg.render("index", &json!(Resume {
        name: "Wenfeng Wang",
        email: "kalot.wang@gmail.com",
        ens: "tolak.eth",
        twitter: "tolak_eth",
        github: "https://github.com/tolak"
    })).unwrap()
}
