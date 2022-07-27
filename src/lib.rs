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
            <div class="container mb-5">
                <div class="row">
                    <div class="col-sm">
                    </div>
                    <div class="col-sm-6 bg-light">
                        // About me
                        <div class="mt-5">
                            <i class="fa fa-quote-left fa-2x fa-pull-left" aria-hidden="true"></i>
                            <h2>{ "About me" }</h2>
                        </div>
                        // Header image and social buttons
                        <div class="row mt-5 mb-2">
                            <div class="col mx-auto text-center">
                                <img class="rounded-circle z-depth-2" src="/images/morty.jpg" width="180" height="180" data-holder-rendered="true" />
                                <h5 class="mt-2">{ "Wenfeng Wang | tolak.eth" }</h5>
                                <div class="btn-group mt-2 row">
                                    <div class="col">
                                        <a href="https://twitter.com/tolak_eth" target="_blank" class="btn btn-social-icon btn-twitter"><i class="fa fa-twitter"></i></a>
                                    </div>
                                    <div class="col">
                                        <a href="https://github.com/tolak" target="_blank" class="btn btn-social-icon btn-github"><i class="fa fa-github"></i></a>
                                    </div>
                                </div>
                            </div>
                        </div>
                        // Description
                        <span class="mt-4 text-secondary">
                            { "Core developer @PhalaNetwork, and " }
                            <a href="https://subbridge.io" target="_blank"  class="text-primary">{ "https://subbridge.io" }</a>
                            {", writing Rust/Solidity, building Off-Chain&Multi-Chain technologies." }
                        </span>

                        // Contact me
                        <div class="mt-5 mb-4">
                            <i class="fa fa-quote-left fa-2x fa-pull-left" aria-hidden="true"></i>
                            <h2>{ "Contact me" }</h2>
                        </div>
                        <form>
                            <div class="form-group">
                                <input type="email" class="form-control" id="emailaddressinput" placeholder="name@example.com"/>
                            </div>
                            <div class="mt-4 form-group">
                                <textarea class="form-control" id="messageinput" rows="3"></textarea>
                            </div>
                            <button type="submit" class="mt-4 btn btn-primary">{ "Send" }</button>
                        </form>
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
