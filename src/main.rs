use gio::prelude::*;
use gtk::prelude::*;

use std::env;
use std::{cell::RefCell, rc::Rc};

struct Pane {
    editor: gtk::TextView,
}

impl Pane {
    fn new() -> Pane {
        Pane {
            editor: gtk::TextView::new(),
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
            &pane.editor,
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
            &pane.editor,
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
    columns: Vec<Column>,
}

impl Window {
    fn new(app: &gtk::Application) -> Rc<RefCell<Window>> {
        let window = gtk::ApplicationWindow::new(app);
        window.set_default_size(1024, 768);
        window.set_title("emma");

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
        window.add(&hbox);

        let r = Rc::new(RefCell::new(Window {
            window,
            layout: hbox,
            columns: vec![column],
        }));

        let copy = r.clone();
        r.borrow_mut()
            .window
            .connect_key_press_event(move |_, key| {
                copy.borrow_mut().on_key_press(key)
            });

        r
    }

    fn show(&self) {
        self.window.show_all();
    }

    fn on_key_press(&mut self, key: &gdk::EventKey) -> Inhibit {
        if key.get_keyval() == '3' as u32 && key.get_state() == gdk::ModifierType::CONTROL_MASK {
            self.add_column();
            Inhibit(true)
        } else if key.get_keyval() == '4' as u32 && key.get_state() == gdk::ModifierType::CONTROL_MASK {
            // TODO
            self.columns[0].add_row();
            Inhibit(true)
        } else {
            Inhibit(false)
        }
    }

    fn add_column(&mut self) {
        let column = Column::new();
        self.layout.pack_start(
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
        Some("org.gtkrsnotes.demo"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    app.connect_activate(|app| {
        let window = Window::new(app);

        window.borrow().show();
    });
    app.run(&env::args().collect::<Vec<_>>());
}
