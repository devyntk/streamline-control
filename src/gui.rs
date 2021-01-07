use druid::commands::{CLOSE_WINDOW, QUIT_APP};
use druid::widget::{Button, Flex, Label};
use druid::{AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, ExtEventSink, Lens, Selector, Target, Widget, WidgetExt, WindowDesc, WindowId, Handled};

use crate::update::{do_update, fetch_is_new, ReleaseStatus};
use crate::server::start_server;

use std::thread;
use tokio::sync::oneshot::{channel, Sender};
use std::net::SocketAddr;
use log::debug;

const START_UPDATE_CHECK: Selector = Selector::new("streamline-control.start-check");
const UPDATE_FOUND: Selector<String> = Selector::new("streamline-control.update-found");
const NO_UPDATE: Selector = Selector::new("streamline-control.no-update-found");
const START_DO_UPDATE: Selector = Selector::new("streamline-control.do-updates");
const UPDATE_FINISHED: Selector = Selector::new("streamline-control.update-finished");
const UPDATE_ERROR: Selector<String> = Selector::new("streamline-control.update-error");
const OPEN_QUIT_CONFIRM: Selector = Selector::new("streamline-control.quit-confirm-open");
pub const SERVER_START: Selector<SocketAddr> = Selector::new("streamline-control.server-start");
pub const UPDATE_STATUS: Selector<String> = Selector::new("streamline-control.update-status");

pub fn run_ui() {
    let main_window_id = WindowId::next();
    let mut main_window = WindowDesc::new(ui_builder)
        .window_size((300.0, 160.0))
        .title("Streamline Server Control");
    main_window.id = main_window_id;

    let (tx, rx) = channel();

    let inital_state = GUIState {
        status: "Server Not Running".into(),
        feedback: "".into(),
        found_update: false,
        update_button: "Check for Updates".into(),
        url: None,
        ready_to_quit: false
    };

    let app = AppLauncher::with_window(main_window);
    let handle = app.get_external_handle();

    thread::spawn(move || {
        start_server(handle, rx)
    });

    let delegate = Delegate {
        eventsink: app.get_external_handle(),
        main_window: main_window_id,
        shutdown_signal: Some(tx),
    };

    app.delegate(delegate)
        .launch(inital_state)
        .expect("Launch failed");
}

#[derive(Clone, Data, Lens)]
struct GUIState {
    status: String,
    feedback: String,
    found_update: bool,
    update_button: String,
    url: Option<String>,
    ready_to_quit: bool
}

fn ui_builder() -> impl Widget<GUIState> {
    let status_label =
        Label::new(|data: &GUIState, _env: &Env| data.status.to_string()).padding(5.0);

    let feedback_label = Label::new(|data: &GUIState, _env: &Env| data.feedback.to_string());

    let quit_button =
        Button::new("Quit")
            .padding(5.0)
            .on_click(|ctx, data: &mut GUIState, _env| {
                if data.ready_to_quit {
                    let cmd = Command::new(OPEN_QUIT_CONFIRM, (), Target::Auto);
                    ctx.submit_command(cmd);
                } else {
                    let cmd = Command::new(CLOSE_WINDOW, (), Target::Auto);
                    ctx.submit_command(cmd);
                }
            });

    let check_button = Button::new(|data: &GUIState, _env: &Env| data.update_button.to_string())
        .on_click(|ctx, data: &mut GUIState, _env| {
            if data.found_update {
                let cmd = Command::new(START_DO_UPDATE, (), Target::Auto);
                ctx.submit_command(cmd);
            } else {
                let cmd = Command::new(START_UPDATE_CHECK, (), Target::Auto);
                ctx.submit_command(cmd);
            }
        })
        .padding(5.0);

    let open_button = Button::new("Open Browser")
        .on_click(move |_ctx, data: &mut GUIState, _env| match &data.url {
            Some(url) => {
                if webbrowser::open(url.as_str()).is_err(){
                    data.feedback = "Unable to Open Browser".into();
                }
            }
            None => data.feedback = "No URL yet set".into(),
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
    main_window: WindowId,
    shutdown_signal: Option<Sender<()>>
}

impl AppDelegate<GUIState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        target: Target,
        cmd: &Command,
        data: &mut GUIState,
        _env: &Env,
    ) -> Handled {
        debug!("{:?}, {:?}", cmd, target);
        if cmd.is(START_UPDATE_CHECK) {
            data.feedback = "Checking For Updates...".into();
            check_updates(self.eventsink.clone())
        } else if cmd.is(NO_UPDATE) {
            data.feedback = "No Update Found".into();
        } else if let Some(version) = cmd.get(UPDATE_FOUND) {
            data.feedback = format!("New Version Found: {}", version);
            data.found_update = true;
            data.update_button = format!("Update to {}", version);
        } else if let Some(msg) = cmd.get(UPDATE_STATUS) {
            data.feedback = format!("{}", msg);
        } else if let Some(err) = cmd.get(UPDATE_ERROR) {
            data.feedback = format!("Error when checking updates: {}", err);
        } else if cmd.is(START_DO_UPDATE) {
            data.feedback = "Updating App...".into();
            wrapped_do_update(self.eventsink.clone())
        } else if cmd.is(UPDATE_FINISHED) {
            data.feedback = "Update Finished. Please restart the app. ".into();
        } else if cmd.is(OPEN_QUIT_CONFIRM) {
            if data.url.is_some() {
                let tx = self.shutdown_signal.take();
                tx.unwrap().send(()).expect("Error when sending shutdown signal");
            }
            let new_cmd = Command::new(QUIT_APP, (), Target::Auto);
            ctx.submit_command(new_cmd);
        } else if let Some(addr) = cmd.get(SERVER_START) {
            data.status = format!("Server started on port {}", addr.port());
            data.url = Some(format!("http://localhost:{}/", addr.port()));
        } else if cmd.is(CLOSE_WINDOW) {
            if Target::Window(self.main_window) == target {
                // let new_cmd = Command::new(OPEN_QUIT_CONFIRM, (), Target::Auto);
                // ctx.submit_command(new_cmd);
                data.status = "Are you sure you want to quit the app?".into();
                data.feedback = "Click quit again to confirm.".into();
                data.ready_to_quit = true;
                return Handled::Yes
            }
        }
        Handled::No
    }

}
fn check_updates(sink: ExtEventSink) {
    thread::spawn(move || {
        let up_to_date = fetch_is_new();
        match up_to_date {
            Ok(ReleaseStatus::UpToDate) => sink.submit_command(NO_UPDATE, (), Target::Auto),
            Ok(ReleaseStatus::NewVersion(release)) => {
                sink.submit_command(UPDATE_FOUND, release.version, Target::Auto)
            }
            Err(err) => sink.submit_command(UPDATE_ERROR, err.to_string(), Target::Auto),
        }
    });
}

fn wrapped_do_update(sink: ExtEventSink) {
    thread::spawn(move || {
        let has_updated = do_update();
        match has_updated {
            Ok(()) => sink.submit_command(UPDATE_FINISHED, (), Target::Auto),
            Err(err) => sink.submit_command(UPDATE_ERROR, err.to_string(), Target::Auto),
        }
    });
}
