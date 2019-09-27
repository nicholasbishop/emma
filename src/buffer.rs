use gtk::TextBufferExt;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use std::{collections::HashMap, fs, path::{Path, PathBuf}};

#[derive(Debug)]
pub enum BufferKind {
    File,
    Shell,
}

impl BufferKind {
    pub fn from_str(s: &str) -> Option<BufferKind> {
        match s {
            "file" => Some(BufferKind::File),
            "shell" => Some(BufferKind::Shell),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            BufferKind::File => "file",
            BufferKind::Shell => "shell",
        }
    }
}

const BUFFER_ID_LEN: usize = 16;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct BufferId([u8; BUFFER_ID_LEN]);

impl BufferId {
    pub fn random() -> BufferId {
        let s: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(BUFFER_ID_LEN)
            .collect();
        BufferId::from_str(&s).unwrap()
    }

    pub fn from_str(s: &str) -> Option<BufferId> {
        if s.len() == BUFFER_ID_LEN {
            let mut a = [0; BUFFER_ID_LEN];
            for (i, c) in s.chars().enumerate() {
                a[i] = c as u8;
            }
            Some(BufferId(a))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        let BufferId(a) = self;
        a.iter().map(|c| *c as char).collect()
    }
}

#[derive(Debug)]
pub struct Buffer {
    pub id: BufferId,
    pub path: PathBuf,
    pub kind: BufferKind,
    pub text: Option<gtk::TextBuffer>,
}

impl Buffer {
    pub fn open_file(path: &Path) -> Buffer {
        let file = fs::read_to_string(path).unwrap();
        let tags: Option<&gtk::TextTagTable> = None;
        let text = gtk::TextBuffer::new(tags);
        text.set_text(&file);

        Buffer {
            id: BufferId::random(),
            path: path.to_path_buf(),
            kind: BufferKind::File,
            text: Some(text)
        }
    }
}

pub type BufferMap = HashMap<BufferId, Buffer>;
