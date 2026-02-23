mod index_list;
mod launcher_scroll;
mod menu_item_model;
mod modes;
mod scroll;

use crate::launcher_scroll::*;
use crate::scroll::ScrollComponent;

use gtk::DrawingArea;
use gtk::prelude::{BoxExt, DrawingAreaExtManual, GtkWindowExt};
use relm4::gtk::gdk::{self, Display};

use relm4::gtk::CssProvider;
use relm4::gtk::prelude::{EditableExt, EntryExt, OrientableExt, WidgetExt};
use relm4::*;

use gtk4_layer_shell::{Edge, Layer, LayerShell};


#[derive(Debug)]
enum AppMsg {
    Query(String),
    MoveDown,
    MoveUp,
    Enter,
}

struct App {
    scroll: Controller<ScrollComponent<LauncherScrollImpl, ScrollListMessages>>,
}

impl App {
    fn load_theme() -> String {
        let mut css = None;

        if let Ok(home) = std::env::var("HOME") {
            let path = std::path::Path::new(&home)
                .join(".config")
                .join("yappla")
                .join("yappla.css");
            if let Ok(contents) = std::fs::read_to_string(&path) {
                css = Some(contents);
            }
        }

        if css.is_none() {
            if let Ok(contents) = std::fs::read_to_string("./yappla.css") {
                css = Some(contents);
            }
        }

        css.unwrap_or_else(|| include_str!("../theme.css").to_string())
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        #[root]

        #[name(#[allow(unused)] window)]
        gtk::Window {
            init_layer_shell: (),
            set_layer: Layer::Overlay,
            add_css_class: "window",
            set_decorated: false,
            set_exclusive_zone: -1,
            set_anchor: (Edge::Left, false),
            set_anchor: (Edge::Right, false),
            set_anchor: (Edge::Top, false),
            set_anchor: (Edge::Bottom, false),
            set_margin: (Edge::Left, 0),
            set_margin: (Edge::Right, 0),
            set_margin: (Edge::Top, 0),
            set_margin: (Edge::Bottom, 0),
            set_focusable: true,
            set_keyboard_mode: gtk4_layer_shell::KeyboardMode::Exclusive,
            

            set_title: Some("yappla"),
            set_default_size: (300, 200),



            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                
                #[name(#[allow(unused)] entry)]
                gtk::Entry {
                    set_hexpand: true,
                    set_placeholder_text: Some("Enter text"),
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
                    connect_activate[_sender] => move |_| {_sender.input(AppMsg::Enter)},
                    connect_changed[_sender] => move |it| {_sender.input(AppMsg::Query(it.text().to_string()))},
                },
                

                append: model.scroll.widget()
            }
        }
    }

    fn init(
        _: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let provider = CssProvider::new();
        
        provider.load_from_data(&Self::load_theme().as_str());

        gtk::style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let model = App {
            scroll: ScrollComponent::builder().launch(()).detach(),
        };

        let widgets = view_output!();
        widgets.window.grab_focus();
        widgets.entry.grab_focus();

        let key_controller = gtk::EventControllerKey::new();

        let clonned_sender = _sender.clone();

        key_controller.connect_key_pressed(move |_controller, keyval, _keycode, _| match keyval {
            gdk::Key::Escape => {
                std::process::exit(0);
            }
            gdk::Key::Up => {
                clonned_sender.input(AppMsg::MoveUp);
                glib::Propagation::Stop
            }
            gdk::Key::Down => {
                clonned_sender.input(AppMsg::MoveDown);
                glib::Propagation::Stop
            }
            _ => glib::Propagation::Proceed,
        });

        
       
        widgets.window.add_controller(key_controller);
        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _: ComponentSender<Self>) {
        match msg {
            AppMsg::Query(text) => self.scroll.sender().emit(ScrollListMessages::Query(text)),
            AppMsg::MoveDown => self.scroll.sender().emit(ScrollListMessages::MoveDown),
            AppMsg::MoveUp => self.scroll.sender().emit(ScrollListMessages::MoveUp),
            AppMsg::Enter => self.scroll.sender().emit(ScrollListMessages::Enter),
        }
    }
}

fn main() {
    RelmApp::new("com.intbyte.yappla")
        .with_args(Vec::default())
        .run::<App>(0);
}
