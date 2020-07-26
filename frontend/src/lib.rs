// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
mod page;

const LOGIN: &str = "login";
const DASH: &str = "dash";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {

    let base_url = url.to_base_url();
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url.clone()));

    Model {
        ctx: Context {},
        base_url,
        page_id: Some(PageId::init(url, orders)),
    }
}

// ------ ------
//     Model
// ------ ------

struct Model {
    ctx: Context,
    base_url: Url,
    page_id: Option<PageId>,
}

// ------ Context ------

pub struct Context {
}

// ------ PageId ------

#[derive(Copy, Clone, Eq, PartialEq)]
enum PageId {
    Login,
    Dash,
    NotFound
}

impl PageId {
    fn init(mut url: Url, _orders: &mut impl Orders<Msg>) -> Self {
        match url.next_path_part() {
            None => Self::Login,
            Some(LOGIN) => Self::Login,
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
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.page_id = match url.next_path_part() {
                None => Some(PageId::Login),
                Some(LOGIN) => {
                    Some(PageId::Login)
                },
                Some(DASH) => {
                    Some(PageId::Dash)
                }
                Some(_) => None,
            };
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
    vec![
        match model.page_id {
            Some(PageId::Login) => div!["Login"],
            Some(PageId::Dash) => div!["Dash"],
            Some(PageId::NotFound) => div!["404"],
            None => div!["404"],
        },
    ]
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