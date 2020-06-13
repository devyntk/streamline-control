use druid::{WindowDesc, AppLauncher, Widget, WidgetExt, Data, Lens, Env, AppDelegate,
            DelegateCtx, Target, Command, Selector, ExtEventSink};
use druid::widget::{Flex, Button, Label};
use druid::commands::QUIT_APP;

use webbrowser;

use crate::update::{fetch_is_new, UpdateStatus};
use std::{thread, string};

const START_UPDATE_CHECK: Selector<u32> =Selector::new("streamline-control.start-check");
const UPDATE_FOUND: Selector<String> =Selector::new("streamline-control.update-found");
const NO_UPDATE: Selector<u32> =Selector::new("streamline-control.no-update-found");
const START_DO_UPDATE: Selector<u32> =Selector::new("streamline-control.do-updates");
const UPDATE_FINISHED: Selector<u32> =Selector::new("streamline-control.update-finished");
const UPDATE_ERROR: Selector<String> = Selector::new("streamline-control.update-error");

pub fn run_ui () {
    let main_window = WindowDesc::new(ui_builder)
        .window_size((300.0,160.0))
        .title("Streamline Server Control");

    let inital_state = GUIState {
        status: "Server Not Running".into(),
        feedback: "".into(),
        found_update: false
    };

    let app = AppLauncher::with_window(main_window)
        .use_simple_logger();

    let delegate = Delegate {
        eventsink: app.get_external_handle(),
    };

    app.delegate(delegate)
        .launch(inital_state)
        .expect("Launch failed");
}

#[derive(Clone, Data, Lens)]
struct GUIState {
    status: String,
    feedback: String,
    found_update: bool
}

fn ui_builder() -> impl Widget<GUIState> {
    let mut status_label = Label::new(|data : &GUIState, _env: &Env| format!("{}", data.status))
        .padding(5.0);

    let mut feedback_label = Label::new(|data : &GUIState, _env: &Env| format!("{}", data.feedback));

    let quit_button = Button::new("Quit").padding(5.0);

    let check_button = Button::new("Check for Updates")
        .on_click(|ctx, data: &mut GUIState, _env| {
            if data.found_update {
                let cmd = Command::new(START_DO_UPDATE, 0);
                ctx.submit_command(cmd, None);
            } else {
                let cmd = Command::new(START_UPDATE_CHECK, 0);
                ctx.submit_command(cmd, None);
            }
        })
        .padding(5.0);

    let open_button = Button::new("Open Browser")
        .on_click(move |_ctx, data: &mut GUIState, _env| {
            if webbrowser::open("http://localhost").is_err() == true {
                data.feedback = "Unable to Open Browser".into();
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

struct Delegate {
    eventsink: ExtEventSink,
}

impl AppDelegate<GUIState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut GUIState,
        _env: &Env,
    ) -> bool {
        if cmd.is(START_UPDATE_CHECK) {
            data.feedback = "Checking For Updates...".into();
        }
        true
    }
}

fn check_updates(sink: ExtEventSink){
    thread::spawn(move || {
        let up_to_date = fetch_is_new();
        match up_to_date {
            Ok(UpdateStatus::UpToDate) => {
                sink.submit_command(NO_UPDATE, 0, None)
            }
            Ok(UpdateStatus::NewVersion(release)) => {
                sink.submit_command(NO_UPDATE, 0, None)
            }
            Err(err) => {
                sink.submit_command(UPDATE_ERROR, err.to_string(), None)
            }

        }
    });
}