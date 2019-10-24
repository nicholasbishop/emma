mod buffer;
mod command;
mod persistence;

use command::CommandWidget;
use gtk::BoxExt;
use gtk::ContainerExt;
use gtk::GtkWindowExt;
use gtk::Inhibit;
use gtk::TextViewExt;
use gtk::WidgetExt;
use std::{cell::RefCell, path::Path, rc::Rc};
use gio::{ApplicationExt, ApplicationExtManual};
use gtk::CssProviderExt;
use std::env;

struct Pane {
    layout: gtk::Box,
    scroller: gtk::ScrolledWindow,
    editor: gtk::TextView,
    label: gtk::Label,
}

impl Pane {
    fn new() -> Pane {
        let label = gtk::Label::new(None);
        gtk::WidgetExt::set_name(&label, "footer");
        let editor = gtk::TextView::new();
        editor.set_monospace(true);

        let scroller = gtk::ScrolledWindow::new(
            None::<&gtk::Adjustment>,
            None::<&gtk::Adjustment>,
        );
        scroller.add(&editor);

        let spacing = 0;
        let layout = gtk::Box::new(gtk::Orientation::Vertical, spacing);
        let spacing = 0;
        let expand = true;
        let fill = true;
        layout.pack_start(&scroller, expand, fill, spacing);
        let expand = false;
        let fill = false;
        layout.pack_start(&label, expand, fill, spacing);
        Pane {
            layout,
            editor,
            label,
            scroller,
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

pub struct Window {
    window: gtk::ApplicationWindow,
    layout: gtk::Box,
    column_layout: gtk::Box,
    columns: Vec<Column>,
    command: Rc<RefCell<CommandWidget>>,
    active_pane_index: (usize, usize),
}

impl Window {
    fn new(
        app: &gtk::Application,
    ) -> Rc<RefCell<Window>> {
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

        let command = CommandWidget::new();

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
            active_pane_index: (0, 0),
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
        } else if key.get_keyval() == 'b' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            // TODO
            // CommandWidget::choose_buffer(
            //     w.clone(),
            //     w.borrow().command.clone(),
            //     w.borrow().buffers.clone(),
            // );
            Inhibit(true)
        } else if key.get_keyval() == 'o' as u32
            && key.get_state() == gdk::ModifierType::CONTROL_MASK
        {
            CommandWidget::find_file(w.clone(), w.borrow().command.clone());
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

    fn get_active_pane(&self) -> &Pane {
        &self.columns[self.active_pane_index.0].panes[self.active_pane_index.1]
    }

    pub fn show_buffer(&self, buffer_id: &buffer::BufferId) {
        // TODO
        // if let Some(buf) = self.buffers.borrow_mut().get_mut(buffer_id) {
        //     buf.load();
        //     if let Some(text) = &buf.text {
        //         self.get_active_pane().editor.set_buffer(Some(text));
        //     }
        // }
    }

    pub fn open_file(&self, path: &Path) {
        // TODO
        // let buf = buffer::Buffer::open_file(path);
        // persistence::add_buffer(&buf).unwrap();
        // let id = buf.id.clone();
        // self.buffers.borrow_mut().insert(id.clone(), buf);
        // if let Some(buf) = self.buffers.borrow().get(&id) {
        //     if let Some(text) = &buf.text {
        //         self.get_active_pane().editor.set_buffer(Some(text));
        //     }
        // }
    }
}

enum Event {
    AppActivated(gtk::Application),
}

pub struct State {
    //app: gtk::Application,
    buffers: buffer::BufferMap,
    //tx_events: glib::Sender<Event>,
}

fn on_app_activated(app: &gtk::Application, state: &State) {
    let css = gtk::CssProvider::new();
    css.load_from_data(include_bytes!("theme.css")).unwrap();

    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().unwrap(),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    // let window = gtk::ApplicationWindow::new(app);
    // window.show();
    let window = Window::new(app);

    // if let Ok(b) = persistence::load_buffer_list() {
    //     buffers.replace(b);
    // }

    window.borrow().show();
}

fn main() {
    persistence::init_db().unwrap();
    let app = gtk::Application::new(
        Some("me.nicholasbishop.emma"),
        gio::ApplicationFlags::FLAGS_NONE,
    )
    .expect("Application::new failed");
    app.clone().connect_activate(move |app| {
        let state = State {
            buffers: buffer::BufferMap::new(),
        };
        let (tx_events, rx_events) = glib::MainContext::channel::<Event>(glib::PRIORITY_DEFAULT);
        on_app_activated(app, &state);

        rx_events.attach(None, move |event| {
            match event {
                Event::AppActivated(app) => {
                    //on_app_activated(&app, &state);
                }
            }

            glib::Continue(true)
        });
    });
    app.run(&env::args().collect::<Vec<_>>());
}
