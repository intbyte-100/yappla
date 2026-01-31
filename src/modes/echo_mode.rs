use glib::object::Cast;
use relm4::gtk::gio::ListModel;

use crate::{
    index_list::{Index, IndexList},
    menu_item_model::{ActionError, MenuItemModel},
    modes::mode::Mode,
};

pub struct EchoMode {
    pub strings: Vec<String>,
}

impl Mode for EchoMode {
    fn search(&self, request: String) -> ListModel {
        todo!()
    }

    fn model(&self) -> ListModel {
        let indecies = IndexList::new();
        indecies.set_indecies((0..(self.strings.len())).map(|i| i as u32).collect());
        indecies.upcast()
    }

    fn get_menu_item_model<'a>(&'a self, item: &Index) -> &'a dyn MenuItemModel {
        &self.strings[item.index() as usize]
    }
}


impl MenuItemModel for String {
    fn name<'a>(&'a self) -> &'a String {
        &self
    }

    fn run_action(&self) -> Result<(), ActionError> {
        println!("{}", self);
        Ok(())
    }
}