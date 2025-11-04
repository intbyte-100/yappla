mod scroll;

use std::cell::Cell;
use std::sync::atomic::AtomicI32;

use crate::scroll::{ScrollComponent, ScrollElement, ScrollImpl, ScrollSettings};
use glib::object::{Cast, ObjectExt};
use glib::types::StaticType;
use gtk::prelude::{BoxExt, GtkWindowExt};
use relm4::gtk::gdk::Display;
use relm4::gtk::gio::ListStore;
use relm4::gtk::prelude::{EntryExt, GestureSingleExt, ListItemExt, OrientableExt, WidgetExt};
use relm4::gtk::{CssProvider, ListItem, NoSelection, SignalListItemFactory};
use relm4::*;

#[derive(Default)]
struct MyScrollImpl;

static COUNTER: AtomicI32 = AtomicI32::new(0);

impl ScrollImpl for MyScrollImpl {
    fn setup() -> ScrollSettings {
        let list_store = ListStore::with_type(ScrollElement::static_type());
        let selection = NoSelection::new(Some(list_store.clone()));

        
        for i in 1..101 {
            list_store.append(&ScrollElement::new(format!("{i}th element").as_str()));
        }
        ScrollSettings {
            list_store,
            selection,
        }
    }

    fn setup_element(_: &SignalListItemFactory, item: &ListItem) {
        let gesture = gtk::GestureClick::new();

        view! {
            gtk_box = gtk::Box {
                add_controller: gesture.clone(),
                set_height_request: 20,
                set_margin_top: 0,
                set_margin_bottom: 0,
                #[name = "label"]
                gtk::Label {}
            }
        };

        let current = COUNTER.load(std::sync::atomic::Ordering::Relaxed);
        COUNTER.store(current +1, std::sync::atomic::Ordering::Relaxed);
        
        println!("{current}");
        
        let label = gtk_box.clone();
        gesture.connect_pressed(move |_gesture, _n_press, _x, _y| {
            label.add_css_class("row-focused");
        });

        item.set_child(Some(&gtk_box));
    }

    fn bind_element(_: &SignalListItemFactory, item: &ListItem) {
        let gtk_box = item.child()
            .unwrap()
            .downcast::<gtk::Box>()
            .unwrap();
        
        let text: glib::GString = item.item().unwrap().property("string");

        gtk_box
            .iter_children()
            .nth(0)
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
            .set_text(text.as_str());
    }
}

struct App {
    scroll: Controller<ScrollComponent<MyScrollImpl>>,
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
                    set_height_request: 100,
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
        provider.load_from_data(
            "
            box.row-focused {
                background-color: #3399FF;
                color: white;
            }
            

        ",
        );
        
        
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

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {}
}

fn main() {
    RelmApp::new("relm4.example.factory")
        .with_args(Vec::default())
        .run::<App>(0);
}
