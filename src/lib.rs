#![recursion_limit = "512"]
#[warn(missing_debug_implementations, missing_docs)]

mod components;

use yew::prelude::*;
// use components::modal::{Modal, get_modal_by_id};

enum Action {
    FollowTwitter,
    SendEmail,
}

#[derive(Properties, Clone, PartialEq, Debug)]
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
            <>
            <div class="container">
                <div class="row">
                    <div class="col-sm">
                    </div>
                    <div class="col-sm-6">
                        // <button onclick={link.callback(|_| Action::FollowTwitter)}>{ "Follow" }</button>
                        <div class="card border-light">
                            <div class="card-header border-light">{ "About me" }</div>
                            <div class="row">
                                <div class="col"></div>
                                <div class="col-6">
                                    <img class="card-img-top rounded-circle z-depth-2" src="/images/morty.jpg" width="180" height="180" data-holder-rendered="true" />
                                </div>
                                <div class="col"></div>
                            </div>
                            <div class="card-body">
                                <h5 class="card-title">{ "Wenfeng Wang | tolak.eth" }</h5>
                                <p class="card-text">{ "Core developer @PhalaNetwork, and https://subbridge.io, writing Rust/Solidity, building Off-Chain&Multi-Chain technologies." }</p>
                                <div class="btn-group row">
                                    <div class="col">
                                        <a href="https://twitter.com/tolak_eth" target="_blank" class="btn btn-dark" width="120px">{ "Twitter" }</a>
                                    </div>
                                    <div class="col">
                                        <a href="https://github.com/tolak" target="_blank" class="btn btn-dark" width="120px">{ "Github" }</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-sm"></div>
                </div>
            </div>
            </>
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
