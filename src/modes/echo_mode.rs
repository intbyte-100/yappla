use std::{
    cell::RefCell,
    io::{self, BufRead},
};

use glib::object::Cast;
use relm4::gtk::gio::ListModel;
use strsim::{jaro_winkler, normalized_levenshtein};

use crate::{
    index_list::{Index, IndexList},
    menu_item_model::{ActionError, MenuItemModel},
    modes::mode::Mode,
};

pub struct EchoMode {
    pub strings: Vec<String>,
    indecies_buffer: RefCell<Vec<(u32, f64)>>,
    lower: Vec<String>,
    model: IndexList,
}

impl EchoMode {
    pub fn new() -> Self {
        let stdin = io::stdin();
        let handle = stdin.lock();

        let strings: Vec<String> = handle.lines().filter_map(Result::ok).collect();

        Self {
            lower: strings.iter().map(|it| it.to_lowercase()).collect(),
            indecies_buffer: RefCell::from(Vec::with_capacity(strings.len())),
            model: IndexList::with_capacity(strings.len()),
            strings,
        }
    }
}

impl Mode for EchoMode {
    fn search(&self, query: String) -> ListModel {
        

        if query.is_empty() {
            self.model.set_indecies((0..(self.strings.len() as u32)).into_iter());
            return self.model.clone().upcast();
        }

        let query_lower = query.to_lowercase();
        let query_len = query_lower.len();

        let entries = self
            .strings
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let item_lower = &self.lower[index];
                let score = if query_len <= 3 {
                    jaro_winkler(&item_lower, &query_lower)
                } else {
                    normalized_levenshtein(&item_lower, &query_lower)
                        + item_lower
                            .contains(&query_lower)
                            .then(|| 0.5)
                            .unwrap_or(0.0)
                };
                (index as u32, score)
            })
            .filter(|(_, score)| *score > 0.3);

        let mut indecies_buffer = self.indecies_buffer.borrow_mut();
        indecies_buffer.clear();
        indecies_buffer.extend(entries);
        indecies_buffer.sort_by(|a, b| b.1.total_cmp(&a.1));

        self.model.set_indecies(indecies_buffer.iter().map(|it| it.0));
        self.model.clone().upcast()
    }

    fn model(&self) -> ListModel {
        self.model.set_indecies((0..(self.strings.len())).map(|i| i as u32));
        self.model.clone().upcast()
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
