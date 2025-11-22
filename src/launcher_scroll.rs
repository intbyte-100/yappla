use std::{cell::RefCell, process::exit, rc::Rc};

use glib::{clone::Downgrade, object::{Cast, ObjectType}, types::StaticType};
use relm4::{gtk::{self, gio::{prelude::ListModelExt, ListStore}, prelude::{ListItemExt, WidgetExt}, ListItem, NoSelection, SignalListItemFactory}, view, RelmIterChildrenExt};

use crate::{launcher_item::Application, scroll::{ScrollBox, ScrollComponentImpl, ScrollSettings, ScrollingData}};


#[derive(Default)]
pub struct LauncherScrollImpl {
    focused: RefCell<Option<ScrollBox>>,
    list_store: RefCell<Option<ListStore>>,
}

impl ScrollComponentImpl for LauncherScrollImpl {
    fn setup(this: Rc<Self>) -> ScrollSettings {
        let list_store = ListStore::with_type(ScrollingData::static_type());
        let selection = NoSelection::new(Some(list_store.clone()));
        *this.list_store.borrow_mut() = Some(list_store.clone());

        for i in 1..101 {
            let data = ScrollingData::new(
                Application::new("Windows is shit".into(), "description".into(), "allacritty -c 'echo windows is shit'".into()).into(),
            );
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
                        .launch().unwrap_or_else(|error| {
                            eprintln!("Error: {}", error);
                            exit(-1);
                        });
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
