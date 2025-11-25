use relm4::gtk::gio;
use relm4::gtk::gio::prelude::ListModelExt;
use relm4::gtk::glib::prelude::*;
use relm4::gtk::glib::subclass::prelude::*;
use std::cell::RefCell;

mod index_imp {
    use glib::Object;
    use glib::object::ObjectExt;
    use glib::subclass::prelude::*;
    use relm4::gtk::glib::{self, Properties};
    use std::cell::Cell;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::Index)]
    pub struct Index {
        #[property(get, set)]
        pub index: Cell<u32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Index {
        const NAME: &'static str = "IndexWrapper";
        type Type = super::Index;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for Index {}
}

glib::wrapper! {
    pub struct Index(ObjectSubclass<index_imp::Index>);
}

impl Index {
    pub fn new(index: u32) -> Self {
        let index_wrapper: Self = glib::Object::builder().build();
        index_wrapper.imp().index.set(index);
        index_wrapper
    }
}
mod index_list_imp {
    use super::*;

    #[derive(Default)]
    pub struct IndexList {
        pub items: RefCell<Vec<u32>>,
        pub gobject_pool: RefCell<Vec<Index>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IndexList {
        const NAME: &'static str = "IndexList";
        type Type = super::ListModel;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for IndexList {}

    impl crate::index_list::gio::subclass::prelude::ListModelImpl for IndexList {
        fn item_type(&self) -> glib::Type {
            Index::static_type()
        }

        fn n_items(&self) -> u32 {
            self.items.borrow().len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            let mut pool = self.gobject_pool.borrow_mut();
            let items = self.items.borrow();

            let index = match items.get(position as usize) {
                Some(index) => index,
                None => return None,
            };

            match pool.pop() {
                Some(index_wrapper) => {
                    index_wrapper.set_index(index);
                    Some(index_wrapper.upcast())
                }
                None => Some(Index::new(*index).upcast()),
            }
        }
    }
}

glib::wrapper! {
    pub struct ListModel(ObjectSubclass<index_list_imp::IndexList>) @implements gio::ListModel;
}

impl ListModel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_indecies(&self, indecies: Vec<u32>) {
        *self.imp().items.borrow_mut() = indecies;

        let new_len = self.imp().items.borrow().len() as u32;
        let list_model: gio::ListModel = self.clone().upcast();
        list_model.items_changed(0, 0, new_len);
    }
}
