use relm4::gtk;

use crate::{index_list::Index, menu_item_model::MenuItemModel};

pub trait Mode {
    fn search(&self, request: String) -> gtk::gio::ListModel;
    fn model(&self) -> gtk::gio:: ListModel;
    fn get_menu_item_model<'a>(&'a self, item: &Index) -> &'a dyn MenuItemModel;
}