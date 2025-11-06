mod launcher_item;
mod scroll;

use std::cell::RefCell;
use std::rc::Rc;

use crate::launcher_item::ShellCommand;
use crate::scroll::{ScrollBox, ScrollComponent, ScrollImpl, ScrollSettings, ScrollingData};
use glib::clone::Downgrade;
use glib::object::{Cast, ObjectType};
use glib::types::StaticType;
use gtk::prelude::{BoxExt, GtkWindowExt};
use relm4::gtk::gdk::Display;
use relm4::gtk::gio::ListStore;
use relm4::gtk::gio::prelude::ListModelExt;
use relm4::gtk::prelude::{EntryExt, ListItemExt, OrientableExt, WidgetExt};
use relm4::gtk::{CssProvider, ListItem, NoSelection, SignalListItemFactory};
use relm4::*;

#[derive(Default)]
struct MyScrollImpl {
    focused: RefCell<Option<ScrollBox>>,
    list_store: RefCell<Option<ListStore>>,
}

impl ScrollImpl for MyScrollImpl {
    fn setup(this: Rc<Self>) -> ScrollSettings {
        let list_store = ListStore::with_type(ScrollingData::static_type());
        let selection = NoSelection::new(Some(list_store.clone()));
        *this.list_store.borrow_mut() = Some(list_store.clone());

        for i in 1..101 {
            let data = ScrollingData::new(ShellCommand::new(format!("alacritty")).into());
            data.set_index(i - 1);
            list_store.append(&data);
        }

        ScrollSettings { selection }
    }

    fn setup_element(this: Rc<Self>, _: &SignalListItemFactory, item: &ListItem) {
        let gesture = gtk::GestureClick::new();

        view! {
            gtk_box = ScrollBox {
                add_controller: gesture.clone(),
                set_height_request: 20,
                set_margin_top: 0,
                set_margin_bottom: 0,
                #[name = "label"]
                gtk::Label {}
            }
        };

        let this = this.downgrade();
        let gtk_clone = gtk_box.downgrade();

        gesture.connect_pressed(move |_gesture, _n_press, _x, _y| {
            let this = this.upgrade().unwrap();
            let gtk_box = gtk_clone.upgrade().unwrap();

            if let Some(focused) = this.focused.borrow().as_ref() {
                if gtk_box.as_ptr() == focused.as_ptr() {
                    this.list_store
                        .borrow()
                        .as_ref()
                        .unwrap()
                        .item(gtk_box.index() as u32)
                        .unwrap()
                        .downcast::<ScrollingData>()
                        .unwrap()
                        .launcher_item()
                        .launch();
                }

                focused.remove_css_class("row-focused");
            }

            gtk_box.add_css_class("row-focused");
            this.focused.borrow_mut().replace(gtk_box.clone());
        });

        item.set_child(Some(&gtk_box));
    }

    fn bind_element(_this: Rc<Self>, _: &SignalListItemFactory, item: &ListItem) {
        let gtk_box = item.child().unwrap().downcast::<gtk::Box>().unwrap();
        let scroll_box = gtk_box.clone().downcast::<ScrollBox>().unwrap();
        let data = item.item().unwrap().downcast::<ScrollingData>().unwrap();

        scroll_box.set_index(data.index());

        gtk_box
            .iter_children()
            .nth(0)
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
            .set_text(data.launcher_item().name().as_str());
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
