use std::rc::Rc;

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
    fn setup(this: Rc<Self>) -> ScrollSettings;
    fn setup_element(this: Rc<Self>, factory: &SignalListItemFactory, item: &ListItem);
    fn bind_element(this: Rc<Self>, factory: &SignalListItemFactory, item: &ListItem);
}

pub struct ScrollSettings {
    pub list_store: gtk::gio::ListStore,
    pub selection: gtk::NoSelection,
}

pub struct ScrollComponent<T> where T: ScrollImpl {
    pub list_store: gtk::gio::ListStore,
    pub selection: gtk::NoSelection,
    scroll_impl: Rc<T>
}





impl<T> ScrollComponent<T> where T: ScrollImpl + 'static {
    fn setup_factory(this: Rc<T>) -> gtk::SignalListItemFactory {
        let factory = gtk::SignalListItemFactory::new();
        
        let impl_clone = this.clone();
        factory.connect_setup(move |factory, item| {
            T::setup_element(impl_clone.clone(),  &factory, &item);
        });

        factory.connect_bind(move|factory, item| {
            T::bind_element(this.clone(), &factory, &item);
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
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let scroll_impl = Rc::from(T::default());
        let settings = T::setup(scroll_impl.clone());
        
        let model = ScrollComponent {
            selection: settings.selection,
            list_store: settings.list_store,
            scroll_impl: scroll_impl.clone()
        };

        
        
        let widgets = view_output!();
        
        widgets.list.set_factory(Some(&Self::setup_factory(scroll_impl)));

        ComponentParts { model, widgets }
    }
}
