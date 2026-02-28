use std::{
    cell::RefCell,
    collections::HashSet,
    env::{self, home_dir},
    fs,
    path::PathBuf,
    process::Command,
};

use freedesktop_file_parser::{EntryType, parse};
use glib::object::Cast;
use strsim::{jaro_winkler, normalized_levenshtein};

use crate::{
    index_list::IndexList,
    menu_item_model::{ActionError, MenuItemModel},
    modes::mode::Mode,
};

pub struct AppsMode {
    apps: Vec<Application>,
    lowered_names: Vec<String>,
    indecies_buffer: RefCell<Vec<(u32, f64)>>,
    model: IndexList,
}

impl AppsMode {
    pub fn new() -> Self {
        let search_paths = Self::get_desktop_search_paths();

        let mut apps = Vec::new();

        for dir in &search_paths {
            if !dir.is_dir() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();

                    if path.extension().and_then(|s| s.to_str()) != Some("desktop") {
                        continue;
                    }

                    let content = match fs::read_to_string(&path) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };

                    let desktop = match parse(&content) {
                        Ok(d) => d,
                        Err(_) => continue,
                    };

                    let desktop_entry = match &desktop.entry.entry_type {
                        EntryType::Application(app) => app,
                        _ => continue,
                    };

                    if desktop.entry.no_display.unwrap_or(false) {
                        continue;
                    }

                    let name = desktop.entry.name.default;
                    let exec = desktop_entry.exec.clone().unwrap_or("".to_string());

                    if exec.is_empty() {
                        continue;
                    }

                    apps.push(Application {
                        name: name.to_string(),
                        exec: exec.to_string(),
                    });
                }
            }
        }

        let lowered_names = apps.iter().map(|a| a.name.to_lowercase()).collect();
        let indecies_buffer = RefCell::new(Vec::with_capacity(apps.len()));
        let model = IndexList::with_capacity(apps.len());

        Self {
            apps,
            lowered_names,
            indecies_buffer,
            model,
        }
    }

    fn get_desktop_search_paths() -> HashSet<PathBuf> {
        let mut paths = Vec::new();

        let user_data = env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                home_dir()
                    .unwrap_or_else(|| PathBuf::from("/"))
                    .join(".local/share")
            });

        paths.push(user_data.join("applications"));

        let system_dirs =
            env::var_os("XDG_DATA_DIRS").unwrap_or_else(|| "/usr/local/share:/usr/share".into());

        for dir in env::split_paths(&system_dirs) {
            paths.push(dir.join("applications"));
        }

        paths.push(PathBuf::from("/var/lib/flatpak/exports/share/applications"));
        if let Some(home) = home_dir() {
            paths.push(home.join(".local/share/flatpak/exports/share/applications"));
        }

        paths.into_iter().collect()
    }
}

impl Mode for AppsMode {
    fn search(&self, query: String) -> gtk::gio::ListModel {
        //TODO: refactor. Move the search into a generalized search function

        if query.is_empty() {
            self.model
                .set_indecies((0..(self.apps.len() as u32)).into_iter());
            return self.model.clone().upcast();
        }

        let query_lower = query.to_lowercase();
        let query_len = query_lower.len();

        let entries = self
            .apps
            .iter()
            .enumerate()
            .map(|(index, _)| {
                let item_lower = &self.lowered_names[index];
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

        self.model
            .set_indecies(indecies_buffer.iter().map(|it| it.0));
        self.model.clone().upcast()
    }

    fn filled_model(&self) -> gtk::gio::ListModel {
        self.model
            .set_indecies((0..(self.apps.len())).map(|i| i as u32));
        self.model()
    }

    fn get_menu_item_model<'a>(&'a self, item: &crate::index_list::Index) -> &'a dyn MenuItemModel {
        &self.apps[item.index() as usize]
    }

    fn model(&self) -> gtk::gio::ListModel {
        self.model.clone().upcast()
    }
}

struct Application {
    name: String,
    exec: String,
}

impl MenuItemModel for Application {
    fn name<'a>(&'a self) -> &'a String {
        &self.name
    }

    fn run_action(&self) -> Result<(), ActionError> {
        let patterns = [
            "%f", "%F", "%u", "%U", "%d", "%D", "%n", "%N", "%i", "%c", "%k", "%v", "%m",
        ];

        let exec = patterns
            .iter()
            .fold(self.exec.clone(), |acc, &pattern| acc.replace(pattern, ""));

        Command::new("sh")
            .arg("-c")
            .arg(&exec)
            .spawn()
            .map_err(|err| ActionError {
                command: self.exec.clone(),
                error: format!("Failed to launch application"),
                cause: err,
            })
            .map(|_| ())
    }
}
