use std::{cell::RefCell, rc::Rc};

use glib::{
    clone::Downgrade,
    object::{Cast, CastNone},
};
use gtk::{ListScrollFlags, ScrollInfo, ScrolledWindow, prelude::AdjustmentExt};
use relm4::{
    RelmIterChildrenExt,
    gtk::{
        self, ListItem, NoSelection, SignalListItemFactory,
        gio::prelude::{ListModelExt, ListModelExtManual},
        prelude::{ListItemExt, WidgetExt},
    },
    view,
};

use crate::{
    index_list::Index,
    modes::{apps_mode::AppsMode, echo_mode::EchoMode, mode::Mode},
    scroll::{ScrollBox, ScrollComponent, ScrollComponentImpl, ScrollSettings},
};

#[derive(Debug)]
pub enum ScrollListMessages {
    Query(String),
    MoveDown,
    MoveUp,
    Enter,
}

pub struct LauncherScrollImpl {
    focused: RefCell<Option<u32>>,
    mode: Box<dyn Mode>,
}

impl LauncherScrollImpl {
    fn set_focus(&self, index: u32) {
        let len = self.mode.model().iter::<Index>().len() as u32;

        if len == 0 {
            *self.focused.borrow_mut() = None;
        } else if len <= index {
            *self.focused.borrow_mut() = Some(len - 1);
        } else {
            let old_index = self.focused.borrow_mut().replace(index);

            old_index.map(|it| self.mode.model().items_changed(it, 1, 1));
            self.mode.model().items_changed(index, 1, 1)
        }
    }

    fn set_focus_with_scroll(&self, index: u32, offset: i32, list_view: &gtk::ListView) {
        self.set_focus(index);

        if let Some(focused) = self.focused.borrow().as_ref() {
            let info = ScrollInfo::new();
            info.enables_vertical();

            let index = focused.checked_add_signed(offset).unwrap_or(*focused);

            if index >= self.mode.model().iter::<Index>().len() as u32 {
                let scroll = list_view
                    .parent()
                    .unwrap()
                    .downcast::<ScrolledWindow>()
                    .unwrap();

                let adj = scroll.vadjustment();
                adj.set_value(adj.upper() - adj.page_size());
                return;
            } 

            list_view.scroll_to(index, ListScrollFlags::empty(), Some(info));
        }
    }
}

impl ScrollComponentImpl<ScrollComponent<Self, ScrollListMessages>, ScrollListMessages>
    for LauncherScrollImpl
{
    fn setup(this: Rc<Self>) -> ScrollSettings {
        let list_store = this.mode.filled_model();
        let selection = NoSelection::new(Some(list_store.clone()));

        this.set_focus(0);

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
                if gtk_box.index() == *focused {
                    let index = Index::new(gtk_box.index() as u32);
                    this.mode.get_menu_item_model(&index).run();
                }
            }

            this.set_focus(gtk_box.index());
        });

        item.set_child(Some(&gtk_box));
    }

    fn bind_element(this: Rc<Self>, _: &SignalListItemFactory, item: &ListItem) {
        let gtk_box = item.child().unwrap().downcast::<gtk::Box>().unwrap();
        let scroll_box = gtk_box.clone().downcast::<ScrollBox>().unwrap();
        let index = item.item().unwrap().downcast::<Index>().unwrap();

        scroll_box.set_index(index.virtual_index());

        if let Some(focused) = this.focused.borrow().as_ref() {
            if scroll_box.index() == *focused {
                scroll_box.add_css_class("row-focused");
            }
        }

        gtk_box
            .iter_children()
            .nth(0)
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
            .set_text(this.mode.get_menu_item_model(&index).name());
    }

    fn init() -> Self {
        let mode = std::env::args().nth(1).unwrap_or(String::new());

        let mode: Box<dyn Mode> = match mode.as_str() {
            "echo" => Box::from(EchoMode::new()),
            "apps" => Box::from(AppsMode::new()),
            _ => {
                eprintln!("Error: unknown mode '{}'.", mode);
                eprintln!("Available modes:");
                eprintln!("  apps   Launch application mode");
                eprintln!("  echo   Echo input back to stdout");
                eprintln!();
                eprintln!("Usage:");
                eprintln!("  yappla <mode>");
            
                std::process::exit(-1);
            },
        };

       

        Self {
            focused: Default::default(),
            mode,
        }
    }

    fn update(
        self: Rc<Self>,
        scroll: &mut ScrollComponent<Self, ScrollListMessages>,
        msg: ScrollListMessages,
        _: relm4::ComponentSender<ScrollComponent<Self, ScrollListMessages>>,
    ) {
        match msg {
            ScrollListMessages::Query(string) => {
                *self.focused.borrow_mut() = None;
                scroll.selection = NoSelection::new(Some(self.mode.search(string)));
                self.set_focus_with_scroll(0, 0, &scroll.list_view.as_ref().unwrap());
            }
            ScrollListMessages::MoveDown => {
                let focused = self.focused.borrow().unwrap_or(0);
                self.set_focus_with_scroll(focused + 1, 1, &scroll.list_view.as_ref().unwrap());
            }
            ScrollListMessages::MoveUp => {
                let focused = self.focused.borrow().unwrap_or(0);
                self.set_focus_with_scroll(
                    focused.checked_sub(1).unwrap_or(0),
                    -1,
                    &scroll.list_view.as_ref().unwrap(),
                );
            }
            ScrollListMessages::Enter => {
                let model = self.mode.model();
                let index = model
                    .item(*self.focused.borrow().as_ref().unwrap_or(&0))
                    .and_downcast::<Index>();

                if let Some(index) = index {
                    self.mode.get_menu_item_model(&index).run();
                }
            }
        }
    }
}
