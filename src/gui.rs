use druid::{WindowDesc, AppLauncher, Widget, WidgetExt, Data, Lens};
use druid::widget::{Flex, Button, Label};

use webbrowser;

use crate::update::{fetch_is_new, UpdateStatus};

pub fn run_ui () {
    let main_window = WindowDesc::new(ui_builder)
        .title("Streamline Server Control");

    let inital_state = GUIState {
        status: "Server Not Running".into(),
        feedback: "".into()
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(inital_state)
        .expect("Launch failed");
}

#[derive(Clone, Data, Lens)]
struct GUIState {
    status: String,
    feedback: String
}

fn ui_builder() -> impl Widget<GUIState> {
    let mut status_label = Label::new("Server Status : Not Running").padding(5.0);

    let mut feedback_label = Label::new("");

    let quit_button = Button::new("Quit").padding(5.0);

    let check_button = Button::new("Check for Updates").padding(5.0);

    let open_button = Button::new("Open Browser")
        .on_click(move |_ctx, data: &mut GUIState, _env| {
            if webbrowser::open("http://localhost").is_err() == true {
                // feedback_label.set_text("Unable to Open Browser");
            }
        })
        .padding(5.0);

    Flex::column()
        .with_child(status_label)
        .with_child(feedback_label)
        .with_spacer(10.0)
        .with_child(open_button)
        .with_child(check_button)
        .with_child(quit_button)
}

fn check_updates(){
    let up_to_date = fetch_is_new();
    match up_to_date {
        Ok(UpdateStatus::UpToDate) => {

        }
        Ok(UpdateStatus::NewVersion(release)) => {

        }
        Err(err) => {

        }
    }
}