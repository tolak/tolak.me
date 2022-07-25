/*
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
*/
#![recursion_limit = "512"]
#[warn(missing_debug_implementations, missing_docs)]

use yew::prelude::*;

enum Action {
    FollowTwitter,
    SendEmail,
}

struct Resume {
    name: &'static str,
    email: &'static str,
    ens: &'static str,
    twitter: &'static str,
    github: &'static str,
    twitter_followers: u32,
}

impl Component for Resume {
    type Message = Action;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            name: "Wenfeng Wang",
            email: "kalot.wang@gmail.com",
            ens: "tolak.eth",
            twitter: "tolak_eth",
            github: "https://github.tom/tolak",
            twitter_followers: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Action::FollowTwitter => {
                self.twitter_followers += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            },
            Action::SendEmail => {
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <div>
                    {"Hi 0000000"}<h1>{format!("Hi, this is {}", self.name)}</h1>
                    <h1>{format!("Contact me with email {}", self.email)}</h1>
                    <h1>{format!("I have a cool ENS {}", self.ens)}</h1>
                    <h1>{format!("Go follow my twitter {}", self.twitter)}</h1>
                    <h1>{format!("Or my github if you are a dev {}", self.github)}</h1>

                </div>
                <button onclick={link.callback(|_| Action::FollowTwitter)}>{ "Follow" }</button>
                <p>{ format!("Twitter followers: {:?}", self.twitter_followers) }</p>
            </div>
        }
    }
}

use log::Level;
use wasm_bindgen::prelude::*;
use web_logger::Config;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// This is the entry point for the web app
#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    web_logger::custom_init(Config { level: Level::Info });
    yew::start_app::<Resume>();
    Ok(())
}
