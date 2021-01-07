// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use shared::LoggedUser;

use seed::{prelude::*, *};
mod page;

const LOGIN: &str = "login";
const DASH: &str = "dash";

const STORAGE_KEY: &str = "robotgear_auth";

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
    Login,
    Dash,
    NotFound,
}

impl PageId {
    fn init(mut url: Url, _orders: &mut impl Orders<Msg>, user: Option<&LoggedUser>) -> Self {
        // This is done to get rid of the leading /app on the URL
        url.next_path_part();

        match url.next_path_part() {
            None => match user {
                None => Self::Login,
                Some(_) => Self::Dash,
            },
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
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
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

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![match model.page_id {
        PageId::Login => div!["Login"],
        PageId::Dash => div!["Dash"],
        PageId::NotFound => div!["404"],
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
