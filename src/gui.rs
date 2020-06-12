use druid::{WindowDesc, AppLauncher, Widget, WidgetExt};
use druid::widget::{Flex, Button};

use webbrowser;

use crate::update::{fetch_is_new, UpdateStatus};

pub fn run_ui () {
    let main_window = WindowDesc::new(ui_builder)
        .title("Streamline Server Control");
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(data)
        .expect("Launch failed");
}

fn ui_builder() -> impl Widget<u32> {
    let quit_button = Button::new("Quit").padding(5.0);

    Flex::column()
        .with_child(quit_button)
}

// pub fn build_ui(ui: &UI) {
//
//     let mut window = Window::new(&ui, "Streamline Server Control", 200, 200, WindowType::NoMenubar);
//
//     // Create a vertical layout to hold the controls
//     let mut vbox = VerticalBox::new(&ui);
//     vbox.set_padded(&ui, true);
//
//     let mut update_button = Button::new(&ui, "Check For Updates");
//     update_button.on_clicked(&ui, check_updates);
//
//     let mut open_button = Button::new(&ui, "Open Browser");
//
//     let mut quit_button = Button::new(&ui, "Quit");
//     quit_button.on_clicked(&ui, {
//         let ui = ui.clone();
//         move |_| {
//             ui.quit();
//         }
//     });
//
//     // Create a new label. Note that labels don't auto-wrap!
//     let mut label_text = String::from("Server is not Running");
//     let label = Label::new(&ui, &label_text);
//
//     vbox.append(&ui, label, LayoutStrategy::Stretchy);
//     vbox.append(&ui, open_button.clone(), LayoutStrategy::Stretchy);
//     vbox.append(&ui, update_button, LayoutStrategy::Stretchy);
//     vbox.append(&ui, quit_button, LayoutStrategy::Stretchy);
//
//     // Actually put the button in the window
//     window.set_child(&ui, vbox);
//     // Show the window
//     window.show(&ui);
//
//     open_button.on_clicked(&ui, {
//         let ui = ui.clone();
//         move |_| {
//             if webbrowser::open("http://localhost").is_err() == true {
//                 window.modal_err(&ui, "Error opening browser", "")
//             }
//         }
//     });
// }

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