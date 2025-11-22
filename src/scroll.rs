use std::cell::Ref;
use std::rc::Rc;

use glib::subclass::types::ObjectSubclassIsExt;
use gtk::glib;
use relm4::gtk::prelude::{BoxExt, WidgetExt};
use relm4::gtk::{ListItem, SignalListItemFactory};
use relm4::*;

use crate::launcher_item::LauncherItem;

mod scroll_data_imp {
    use glib::Object;
    use glib::object::ObjectExt;
    use glib::subclass::prelude::*;
    use relm4::gtk::glib::{self, Properties};

    use std::cell::{Cell, RefCell};

    use crate::launcher_item::LauncherItem;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::ScrollingData)]
    pub struct ScrollingData {
        pub item: RefCell<LauncherItem>,
        #[property(get, set)]
        pub index: Cell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScrollingData {
        const NAME: &'static str = "ScrollData";
        type Type = super::ScrollingData;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ScrollingData {}
}

mod scroll_imp {
    use glib::object::ObjectExt;
    use glib::subclass::prelude::*;
    use std::cell::RefCell;

    use glib::{
        Properties,
        subclass::{object::ObjectImpl, types::ObjectSubclass},
    };
    use relm4::gtk::{
        self,
        subclass::{box_::BoxImpl, widget::WidgetImpl},
    };

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::ScrollBox)]
    pub struct ScrollBox {
        #[property(get, set)]
        pub index: RefCell<i32>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScrollBox {
        const NAME: &'static str = "ScrollBox";
        type Type = super::ScrollBox;
        type ParentType = gtk::Box;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ScrollBox {}
    impl WidgetImpl for ScrollBox {}
    impl BoxImpl for ScrollBox {}
}

glib::wrapper! {
    pub struct ScrollingData(ObjectSubclass<scroll_data_imp::ScrollingData>);
}

glib::wrapper! {
    pub struct ScrollBox(ObjectSubclass<scroll_imp::ScrollBox>)
        @extends gtk::Box, gtk::Widget, glib::InitiallyUnowned,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for ScrollBox {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

impl ContainerChild for ScrollBox {
    type Child = gtk::Widget;
}

impl RelmContainerExt for ScrollBox {
    fn container_add(&self, widget: &impl AsRef<gtk::Widget>) {
        self.append(widget.as_ref());
    }
}

impl ScrollingData {
    pub fn new(item: LauncherItem) -> Self {
        let data: Self = glib::Object::builder().build();
        *data.imp().item.borrow_mut() = item;
        data
    }
    
    pub fn launcher_item(&self) -> Ref<'_, LauncherItem> {
        self.imp().item.borrow()
    }
}

pub trait ScrollComponentImpl {
    fn setup(this: Rc<Self>) -> ScrollSettings;
    fn setup_element(this: Rc<Self>, factory: &SignalListItemFactory, item: &ListItem);
    fn bind_element(this: Rc<Self>, factory: &SignalListItemFactory, item: &ListItem);
}

pub struct ScrollSettings {
    pub selection: gtk::NoSelection,
}

pub struct ScrollComponent<T>
where
    T: ScrollComponentImpl,
{
    pub selection: gtk::NoSelection,
    _scroll_impl: Rc<T>,
}

impl<T> ScrollComponent<T>
where
    T: ScrollComponentImpl + 'static,
{
    fn setup_factory(this: Rc<T>) -> gtk::SignalListItemFactory {
        let factory = gtk::SignalListItemFactory::new();

        let impl_clone = this.clone();
        factory.connect_setup(move |factory, item| {
            T::setup_element(impl_clone.clone(), &factory, &item);
        });

        factory.connect_bind(move |factory, item| {
            T::bind_element(this.clone(), &factory, &item);
        });

        factory
    }
}

#[relm4::component(pub)]
impl<T> relm4::SimpleComponent for ScrollComponent<T>
where
    T: ScrollComponentImpl + Default + 'static,
{
    type Input = ();
    type Output = ();
    type Init = ();

    view! {
        #[root]
        gtk::Box {
            gtk::ScrolledWindow {
                set_vexpand: true,
                set_hexpand: true,
                set_policy: (gtk::PolicyType::Automatic, gtk::PolicyType::Automatic),

                #[name = "list"]
                gtk::ListView {
                    add_css_class: "listview",
                    set_show_separators: false,
                    set_model: Some(&model.selection),
                    set_can_focus: false
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let scroll_impl = Rc::from(T::default());
        let settings = T::setup(scroll_impl.clone());

        let model = ScrollComponent {
            selection: settings.selection,
            _scroll_impl: scroll_impl.clone(),
        };

        let widgets = view_output!();

        widgets
            .list
            .set_factory(Some(&Self::setup_factory(scroll_impl)));

        ComponentParts { model, widgets }
    }
}
