use relm4::gtk::{prelude::*, ListItem, ListStore, SignalListItemFactory};
use relm4::*;

mod imp {
    use glib::Object;
    use glib::object::ObjectExt;
    use glib::subclass::prelude::*; 
    use relm4::gtk::glib::{self, Properties};
    use std::cell::RefCell;

    #[derive(Default, Properties)]
    #[properties(wrapper_type = super::ScrollElement)]
    pub struct ScrollElement {
        #[property(get, set)]
        pub string: RefCell<String>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ScrollElement {
        const NAME: &'static str = "ScrollElement";
        type Type = super::ScrollElement;
        type ParentType = Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for ScrollElement {}
}

glib::wrapper! {
    pub struct ScrollElement(ObjectSubclass<imp::ScrollElement>);
}

impl ScrollElement {
    pub fn new(string: &str) -> Self {
        glib::Object::builder().property("string", string).build()
    }
}

pub trait ScrollImpl {
    fn setup() -> ScrollSettings;
    fn setup_element(factory: &SignalListItemFactory, item: &ListItem);
    fn bind_element(factory: &SignalListItemFactory, item: &ListItem);
}

pub struct ScrollSettings {
    pub list_store: gtk::gio::ListStore,
    pub selection: gtk::NoSelection,
}

pub struct ScrollComponent<T> where T: ScrollImpl {
    pub list_store: gtk::gio::ListStore,
    pub selection: gtk::NoSelection,
    scroll_impl: T
}





impl<T> ScrollComponent<T> where T: ScrollImpl{
    fn setup_factory() -> gtk::SignalListItemFactory {
        let factory = gtk::SignalListItemFactory::new();
        
        factory.connect_setup(|factory, item| {
            T::setup_element(&factory, &item);
        });

        factory.connect_bind(|factory, item| {
            T::bind_element(&factory, &item);
        });

        factory
    }
    
}

#[relm4::component(pub)]
impl<T> relm4::SimpleComponent for ScrollComponent<T> where T: ScrollImpl + Default +'static {
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
    
                gtk::ListView {
                    set_show_separators: false,
                    set_model: Some(&model.selection),
                    set_can_focus: false,
                    set_factory: Some(&Self::setup_factory())
                }
            }
        }
        
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let settings = T::setup();
        
        let model = ScrollComponent {
            selection: settings.selection,
            list_store: settings.list_store,
            scroll_impl: T::default()
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
