use druid::widget::{Button, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

fn main() {
    // This describes our window: its title and initial size.
    let main_window = WindowDesc::new(|| build_ui())
        .title(LocalizedString::new("mini-browser-window-title").with_placeholder("Mini Browser"))
        .window_size((400.0, 100.0));

    // This launches our application with AppState as the initial data.
    AppLauncher::with_window(main_window)
        .launch(AppState { address: String::new() })
        .expect("Failed to launch application");
}

//This is where you will add functionality to the browser, improving the user experience.
fn build_ui() -> impl Widget<AppState> {
    // This creates a horizontal layout.
    let layout = Flex::row();

    // New Home button added here
    let home_button = Button::new("Home").on_click(|_ctx, _data, _env| {
        println!("Home button clicked!");
    });
    
    // New buttons added here
    let back_button = Button::new("Back").on_click(|_ctx, _data, _env| {
        println!("Back button clicked!");
    });

    // New Forward button here.
    let forward_button = Button::new("Forward").on_click(|_ctx, _data, _env| {
        println!("Forward button clicked!");
    });    

    // We add a TextBox to our layout. This will be our address bar.
    let address_bar = TextBox::new()
        .with_placeholder("Enter URL here")
        .fix_width(200.0)
        .lens(AppState::address);

    // We add a button labeled "Go" next to the address bar.
    let go_button = Button::new("Go").on_click(|_ctx, _data, _env| {
        println!("Go button clicked!");
    });

    // Add the buttons to the layout
    layout
        .with_child(home_button)    
        .with_child(back_button)
        .with_child(forward_button)
        .with_flex_child(address_bar, 1.0) // We want the address bar to take up extra space if available
        .with_child(go_button)
}

// This is our application's state.
#[derive(Clone, Data, Lens, Default)]
struct AppState {
    address: String,
}
