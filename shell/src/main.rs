use gtk::{glib::DateTime, prelude::*};
use gtk_layer_shell::{Edge, Layer, LayerShell};
use relm4::prelude::*;

#[derive(Debug)]
enum TopBarMsg {
    ActionStart,
    ActionCenter,
    ActionEnd,
}

struct TopBarModel {
    date: DateTime,
}

#[relm4::component]
impl SimpleComponent for TopBarModel {
    type Init = ();
    type Input = TopBarMsg;
    type Output = ();

    view! {
        gtk::Window {
            init_layer_shell: (),
            set_namespace: Some("gnesque-shell"),
            set_layer: Layer::Overlay,
            #[watch]
            auto_exclusive_zone_enable: (),
            set_anchor: (Edge::Left, true),
            set_anchor: (Edge::Right, true),
            set_anchor: (Edge::Top, true),
            set_anchor: (Edge::Bottom, false),
            set_css_classes: &["osd", "topbar"],


            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,

                gtk::Button::with_label("Overview") {
                    set_css_classes: &["flat", "pill", "compact"],
                    connect_clicked => TopBarMsg::ActionStart,
                },

                gtk::Box { set_hexpand: true },

                gtk::Button {
                    set_css_classes: &["flat", "pill", "compact"],
                    #[watch]
                    set_label: &model.date.format("%d.%m.%Y %H:%M").unwrap_or("<error>".into()).to_string(),
                    connect_clicked => TopBarMsg::ActionCenter,
                },

                gtk::Box { set_hexpand: true },

                gtk::Button::with_label("System Tray") {
                    set_css_classes: &["flat", "pill", "compact"],
                    connect_clicked => TopBarMsg::ActionEnd,
                },
            }
        }
    }

    fn init(_: (), window: gtk::Window, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = TopBarModel {
            date: DateTime::now_utc().unwrap(),
        };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: TopBarMsg, _sender: ComponentSender<Self>) {
        match message {
            TopBarMsg::ActionStart => println!("Left button clicked"),
            TopBarMsg::ActionCenter => self.date = DateTime::now_utc().unwrap(),
            TopBarMsg::ActionEnd => println!("Right button clicked"),
        }
    }
}

fn main() {
    let app = RelmApp::new("gnesque.shell");

    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("style.css"));

    gtk::style_context_add_provider_for_display(
        &gtk::gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    app.run::<TopBarModel>(());
}
