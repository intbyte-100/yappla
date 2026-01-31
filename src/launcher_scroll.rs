use std::{cell::RefCell, rc::Rc};

use glib::{
    clone::Downgrade,
    object::{Cast, ObjectType},
};
use relm4::{
    RelmIterChildrenExt,
    gtk::{
        self, ListItem, NoSelection, SignalListItemFactory,
        prelude::{ListItemExt, WidgetExt},
    },
    view,
};


use crate::{
    modes::mode::Mode,
    index_list::Index,
    modes::echo_mode::EchoMode,
    scroll::{ScrollBox, ScrollComponentImpl, ScrollSettings},
};

pub struct LauncherScrollImpl {
    focused: RefCell<Option<ScrollBox>>,
    mode: Box<dyn Mode>,
}

impl ScrollComponentImpl for LauncherScrollImpl {
    fn setup(this: Rc<Self>) -> ScrollSettings {
        let list_store = this.mode.model();
        let selection = NoSelection::new(Some(list_store.clone()));

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
                    // TODO: replace the creation of a new index object with the use of the inner Index from gtk_box.
                    let index = Index::new(gtk_box.index() as u32);
                    this.mode.get_menu_item_model(&index).run_action();
                }

                focused.remove_css_class("row-focused");
            }

            gtk_box.add_css_class("row-focused");
            this.focused.borrow_mut().replace(gtk_box.clone());
        });

        item.set_child(Some(&gtk_box));
    }

    fn bind_element(this: Rc<Self>, _: &SignalListItemFactory, item: &ListItem) {
        let gtk_box = item.child().unwrap().downcast::<gtk::Box>().unwrap();
        let scroll_box = gtk_box.clone().downcast::<ScrollBox>().unwrap();
        let index = item.item().unwrap().downcast::<Index>().unwrap();

        scroll_box.set_index(index.index() as i32);

        gtk_box
            .iter_children()
            .nth(0)
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
            .set_text(this.mode.get_menu_item_model(&index).name());
    }

    fn init() -> Self {
        Self {
            mode: Box::from(EchoMode {
                strings: (0..1000).map(|it| it.to_string()).collect(),
            }),
            focused: Default::default(),
        }
    }
}
