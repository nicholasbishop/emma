use crate::{
    buffer::{buffer_from_name, BufferMap},
    TxEvents,
    Window,
};
use gtk::Inhibit;
use gtk::TextBufferExt;
use gtk::TextTagExt;
use gtk::TextTagTableExt;
use gtk::TextViewExt;
use gtk::WidgetExt;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

enum Command {
    None,
    ChooseBuffer,
    FindFile,
}

pub struct CommandWidget {
    editor: gtk::TextView,
    window: Option<Rc<RefCell<Window>>>,
    buffers: Option<Rc<RefCell<BufferMap>>>,
    command: Command,
    tx_events: TxEvents,
}

impl CommandWidget {
    pub fn new(tx_events: TxEvents) -> CommandWidget {
        let editor = gtk::TextView::new();
        editor.set_monospace(true);

        let r = CommandWidget {
            editor,
            window: None,
            command: Command::None,
            buffers: None,
            tx_events,
        };

        // TODO
        // r
        //     .editor
        //     .get_buffer()
        //     .unwrap()
        //     .connect_changed(move |_| {
        //         Self::on_command_changed(r2.clone());
        //     });

        // TODO
        // let r2 = r.clone();
        // r.borrow().editor.connect_key_press_event(move |_, key| {
        //     Self::on_key_press(r2.clone(), key)
        // });
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
        let left_gravity = true;
        buffer.create_mark(
            Some("input-start"),
            &buffer.get_end_iter(),
            left_gravity,
        );
    }

    pub fn find_file(w: Rc<RefCell<Window>>, c: Rc<RefCell<CommandWidget>>) {
        c.borrow_mut().window = Some(w);
        c.borrow_mut().command = Command::FindFile;
        c.borrow().editor.grab_focus();
        c.borrow().set_prompt("Find file: ");
        let buffer = c.borrow().editor.get_buffer().unwrap();
        let left_gravity = false;
        buffer.create_mark(
            Some("input-end"),
            &buffer.get_end_iter(),
            left_gravity,
        );
    }

    pub fn choose_buffer(
        w: Rc<RefCell<Window>>,
        c: Rc<RefCell<CommandWidget>>,
        buffers: Rc<RefCell<BufferMap>>,
    ) {
        c.borrow_mut().window = Some(w);
        c.borrow_mut().buffers = Some(buffers);
        c.borrow_mut().command = Command::ChooseBuffer;
        c.borrow().editor.grab_focus();
        c.borrow().set_prompt("Choose buffer: ");

        let left_gravity = false;
        let buffer = c.borrow().editor.get_buffer().unwrap();
        buffer.create_mark(
            Some("input-end"),
            &buffer.get_end_iter(),
            left_gravity,
        );
    }

    fn clear(&self) {
        let buffer = self.editor.get_buffer().unwrap();
        buffer.set_text("");
    }

    fn get_input_text(&self) -> Option<String> {
        let buffer = self.editor.get_buffer()?;
        let start = buffer.get_iter_at_mark(&buffer.get_mark("input-start")?);
        let end = buffer.get_iter_at_mark(&buffer.get_mark("input-end")?);
        let include_hidden_chars = false;
        let path = buffer.get_slice(&start, &end, include_hidden_chars)?;
        Some(path.to_string())
    }

    fn end_choose_buffer(&self) -> Option<()> {
        let buf_name = self.get_input_text()?;
        self.clear();
        let buf_id;
        {
            let buffers = &self.buffers.as_ref()?.borrow();
            let buf = buffer_from_name(buffers, &buf_name);
            buf_id = buf?.id.clone();
        }
        self.window.as_ref()?.borrow().show_buffer(&buf_id);
        Some(())
    }

    fn end_find_file(&self) {
        let path = PathBuf::from(self.get_input_text().unwrap());
        self.window.as_ref().unwrap().borrow().open_file(&path);
        self.clear();
    }

    fn on_key_press(
        c: Rc<RefCell<CommandWidget>>,
        key: &gdk::EventKey,
    ) -> Inhibit {
        if key.get_keyval() == gdk::enums::key::Return {
            match c.borrow().command {
                Command::None => {}
                Command::ChooseBuffer => {
                    c.borrow().end_choose_buffer();
                }
                Command::FindFile => c.borrow().end_find_file(),
            }
            Inhibit(true)
        } else {
            Inhibit(false)
        }
    }

    fn on_command_changed(_c: Rc<RefCell<CommandWidget>>) {}
}
