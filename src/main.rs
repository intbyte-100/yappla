mod launcher_item;
mod scroll;
mod mode;
mod launcher_scroll;
mod index_list;


use crate::scroll::ScrollComponent;
use crate::launcher_scroll::*;

use gtk::prelude::{BoxExt, GtkWindowExt};
use relm4::gtk::gdk::Display;

use relm4::gtk::prelude::{EntryExt, OrientableExt, WidgetExt};
use relm4::gtk::CssProvider;
use relm4::*;


struct App {
    scroll: Controller<ScrollComponent<LauncherScrollImpl>>,
}

#[derive(Debug)]
enum AppMsg {}

#[relm4::component]
impl SimpleComponent for App {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Factory example"),
            set_default_size: (300, 100),


            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                gtk::Entry {
                    set_hexpand: true,
                    set_placeholder_text: Some("Enter text"),
                    set_margin_start: 10,
                    set_margin_end: 10,
                    set_margin_top: 10,
                    set_margin_bottom: 10,
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
        provider.load_from_data(include_str!("../theme.css"));

        gtk::style_context_add_provider_for_display(
            &Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let model = App {
            scroll: ScrollComponent::builder().launch(()).detach(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _msg: Self::Input, _sender: ComponentSender<Self>) {}
}

fn main() {
    RelmApp::new("relm4.example.factory")
        .with_args(Vec::default())
        .run::<App>(0);
}
