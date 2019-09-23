mod command;

use gio::ApplicationExt;
use gio::ApplicationExtManual;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::CssProviderExt;
use gtk::GtkWindowExt;
use gtk::Inhibit;
use gtk::TextViewExt;
use gtk::WidgetExt;
use std::{cell::RefCell, env, rc::Rc};

struct Pane {
    layout: gtk::Box,
    editor: gtk::TextView,
    label: gtk::Label,
}

impl Pane {
    fn new() -> Pane {
        let label = gtk::Label::new(None);
        gtk::WidgetExt::set_name(&label, "footer");
        let editor = gtk::TextView::new();
        editor.set_monospace(true);

        let spacing = 0;
        let layout = gtk::Box::new(gtk::Orientation::Vertical, spacing);
        let spacing = 0;
        let expand = true;
        let fill = true;
        layout.pack_start(&editor, expand, fill, spacing);
        let expand = false;
        let fill = false;
        layout.pack_start(&label, expand, fill, spacing);
        Pane {
            layout,
            editor,
            label,
        }
    }
}

struct Column {
    layout: gtk::Box,
    panes: Vec<Pane>,
}

impl Column {
    fn new() -> Column {
        let pane = Pane::new();
        let vbox =
            gtk::Box::new(gtk::Orientation::Vertical, /*spacing=*/ 1);
        vbox.set_homogeneous(true);
        vbox.pack_start(
            &pane.layout,
            /*expand=*/ true,
            /*fill=*/ true,
            /*spacing=*/ 0,
        );
        Column {
            layout: vbox,
            panes: vec![pane],
        }
    }

    fn add_row(&mut self) {
        let pane = Pane::new();
        self.layout.pack_start(
            &pane.layout,
            /*expand=*/ true,
            /*fill=*/ true,
            /*spacing=*/ 0,
        );
        self.layout.show_all();
        self.panes.push(pane);
    }
}

struct Window {
    window: gtk::ApplicationWindow,
    layout: gtk::Box,
    column_layout: gtk::Box,
    columns: Vec<Column>,
    command: Rc<RefCell<command::Command>>,
}

impl Window {
    fn new(app: &gtk::Application) -> Rc<RefCell<Window>> {
        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1024, 768);
        window.set_title("emma");

        let vbox =
            gtk::Box::new(gtk::Orientation::Vertical, /*spacing=*/ 0);

        let column = Column::new();
        let hbox =
            gtk::Box::new(gtk::Orientation::Horizontal, /*spacing=*/ 1);
        hbox.set_homogeneous(true);
        hbox.pack_start(
            &column.layout,
            /*expand=*/ true,
            /*fill=*/ true,
            /*spacing=*/ 0,
        );

        let command = command::Command::new();

        let expand = true;
        let fill = true;
        let spacing = 0;
        vbox.pack_start(&hbox, expand, fill, spacing);
        let expand = false;
        vbox.pack_start(command.borrow().widget(), expand, fill, spacing);
        window.add(&vbox);

        let r = Rc::new(RefCell::new(Window {
            window,
            layout: vbox,
            column_layout: hbox,
            columns: vec![column],
            command,
        }));

        let r2 = r.clone();
        r.borrow().window.connect_key_press_event(move |_, key| {
            Self::on_key_press(r2.clone(), key)
        });

        r
    }

    fn show(&self) {
        self.window.show_all();
    }

    fn on_key_press(w: Rc<RefCell<Window>>, key: &gdk::EventKey) -> Inhibit {
        if key.get_keyval() == '3' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            w.borrow_mut().add_column();
            Inhibit(true)
        } else if key.get_keyval() == '4' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            // TODO
            w.borrow_mut().columns[0].add_row();
            Inhibit(true)
        } else if key.get_keyval() == 'o' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            command::Command::find_file(w.borrow().command.clone());
            Inhibit(true)
        } else {
            Inhibit(false)
        }
    }

    fn add_column(&mut self) {
        let column = Column::new();
        self.column_layout.pack_start(
            &column.layout,
            /*expand=*/ true,
            /*fill=*/ true,
            /*spacing=*/ 0,
        );
        column.layout.show_all();
        self.columns.push(column);
    }
}

fn main() {
    let app = gtk::Application::new(
        Some("me.nicholasbishop.emma"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    app.connect_activate(|app| {
        let css = gtk::CssProvider::new();
        css.load_from_data(include_bytes!("theme.css")).unwrap();

        gtk::StyleContext::add_provider_for_screen(
            &gdk::Screen::get_default().unwrap(),
            &css,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = Window::new(app);

        window.borrow().show();
    });
    app.run(&env::args().collect::<Vec<_>>());
}
