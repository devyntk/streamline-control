// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]
#![allow(dead_code)]

use shared::LoggedUser;

mod generated;
mod page;

use generated::css_classes::C;
use seed::{prelude::*, *};

const LOGIN: &str = "login";
const DASH: &str = "dash";

const STORAGE_KEY: &str = "streamline_auth";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let user = LocalStorage::get(STORAGE_KEY).ok();

    let base_url = url.to_base_url();
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));

    Model {
        ctx: Context {},
        base_url,
        page_id: PageId::init(url, orders, user.as_ref()),
        user,
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    user: Option<LoggedUser>,
    base_url: Url,
    page_id: PageId,
}

// ------ Context ------

pub struct Context {}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum PageId {
    Loading,
    Login,
    Dash,
    NotFound,
}

impl PageId {
    fn init(mut url: Url, _orders: &mut impl Orders<Msg>, _user: Option<&LoggedUser>) -> Self {

        match url.next_path_part() {
            None => Self::Loading,
            Some(LOGIN) => Self::Login,
            Some(DASH) => Self::Dash,
            Some(_) => Self::NotFound,
        }
    }
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    UrlChanged(subs::UrlChanged),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            model.page_id = PageId::init(url, orders, model.user.as_ref());
        }
    }
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    pub fn home(self) -> Url {
        self.base_url()
    }
    // pub fn dash_urls(self) -> page::admin::Urls<'a> {
    //     page::admin::Urls::new(self.base_url().add_path_part(ADMIN))
    // }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![match model.page_id {
        PageId::Login => div!["Login"],
        PageId::Dash => div!["Dash"],
        PageId::NotFound => div!["404"],
        PageId::Loading => div![
            C![
                C.pageloader,
                C.is_active
            ],
            span![
                C![
                    C.title,
                ],
                "Loading Streamline Control"
            ]
        ]
    }]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
