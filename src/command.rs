use crate::Window;
use gtk::Inhibit;
use gtk::TextBufferExt;
use gtk::TextTagExt;
use gtk::TextTagTableExt;
use gtk::TextViewExt;
use gtk::WidgetExt;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

// TODO(nicholasbishop): for now this just implements the "find file"
// command. Once we add a second command this might become a trait or
// something.

pub struct Command {
    editor: gtk::TextView,
    window: Option<Rc<RefCell<Window>>>,
}

impl Command {
    pub fn new() -> Rc<RefCell<Command>> {
        let editor = gtk::TextView::new();
        editor.set_monospace(true);

        let r = Rc::new(RefCell::new(Command {
            editor,
            window: None,
        }));

        let r2 = r.clone();
        r.borrow()
            .editor
            .get_buffer()
            .unwrap()
            .connect_changed(move |_| {
                Self::on_command_changed(r2.clone());
            });
        let r2 = r.clone();
        r.borrow().editor.connect_key_press_event(move |_, key| {
            Self::on_key_press(r2.clone(), key)
        });
        r
    }

    pub fn widget(&self) -> &gtk::TextView {
        &self.editor
    }

    fn set_prompt(&self, prompt: &str) {
        let buffer = self.editor.get_buffer().unwrap();
        buffer.set_text(prompt);
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

    pub fn find_file(w: Rc<RefCell<Window>>, c: Rc<RefCell<Command>>) {
        c.borrow_mut().window = Some(w);
        c.borrow().editor.grab_focus();
        c.borrow().set_prompt("Find file: ");
        let buffer = c.borrow().editor.get_buffer().unwrap();
        let left_gravity = true;
        buffer.create_mark(
            Some("path-start"),
            &buffer.get_end_iter(),
            left_gravity,
        );
        let left_gravity = false;
        buffer.create_mark(
            Some("path-end"),
            &buffer.get_end_iter(),
            left_gravity,
        );
    }

    pub fn choose_buffer(w: Rc<RefCell<Window>>, c: Rc<RefCell<Command>>) {
        c.borrow_mut().window = Some(w);
        c.borrow().editor.grab_focus();
        c.borrow().set_prompt("Choose buffer: ");
    }

    fn clear(&self) {
        let buffer = self.editor.get_buffer().unwrap();
        buffer.set_text("");
    }

    fn get_current_path(&self) -> Option<PathBuf> {
        let buffer = self.editor.get_buffer()?;
        let start = buffer.get_iter_at_mark(&buffer.get_mark("path-start")?);
        let end = buffer.get_iter_at_mark(&buffer.get_mark("path-end")?);
        let include_hidden_chars = false;
        let path = buffer.get_slice(&start, &end, include_hidden_chars)?;
        Some(PathBuf::from(path.as_str()))
    }

    fn on_key_press(c: Rc<RefCell<Command>>, key: &gdk::EventKey) -> Inhibit {
        if key.get_keyval() == gdk::enums::key::Return {
            let path = c.borrow().get_current_path().unwrap();
            c.borrow()
                .window
                .as_ref()
                .unwrap()
                .borrow()
                .open_file(&path);
            c.borrow().clear();
            Inhibit(true)
        } else {
            Inhibit(false)
        }
    }

    fn on_command_changed(_c: Rc<RefCell<Command>>) {}
}
