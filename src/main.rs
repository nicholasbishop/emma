use gio::prelude::*;
use gtk::prelude::*;

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
    command: gtk::TextView,
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

        let command = gtk::TextView::new();
        command.set_monospace(true);

        let expand = true;
        let fill = true;
        let spacing = 0;
        vbox.pack_start(&hbox, expand, fill, spacing);
        let expand = false;
        vbox.pack_start(&command, expand, fill, spacing);
        window.add(&vbox);

        let r = Rc::new(RefCell::new(Window {
            window,
            layout: vbox,
            column_layout: hbox,
            columns: vec![column],
            command,
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
        if key.get_keyval() == '3' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            self.add_column();
            Inhibit(true)
        } else if key.get_keyval() == '4' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            // TODO
            self.columns[0].add_row();
            Inhibit(true)
        } else if key.get_keyval() == 'o' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            self.find_file();
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

    fn find_file(&mut self) {
        self.command.grab_focus();
        let buffer = self.command.get_buffer().unwrap();
        buffer.set_text("Find file: ");
        let prompt_tag = gtk::TextTag::new(None);
        prompt_tag.set_property_editable(false);
        prompt_tag.set_property_foreground_rgba(Some(&gdk::RGBA {
            red: 0.929,
            green: 0.831,
            blue: 0.012,
            alpha: 1.0,
        }));
        buffer.get_tag_table().unwrap().add(&prompt_tag);
        buffer.apply_tag(
            &prompt_tag,
            &buffer.get_start_iter(),
            &buffer.get_end_iter(),
        );
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
