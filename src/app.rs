use rodio::Sink;
use std::collections::VecDeque;
use std::env;
use std::sync::{Arc, Mutex};

use crate::list::ContentList;

pub struct App {
    pub should_quit: bool,
    pub sink: Sink,
    pub songs_list: ContentList,
    pub play_deque: VecDeque<String>,
    pub now_playing: String,
    pub error_message: Option<String>,
}

impl App {
    pub fn new(s: Sink) -> Arc<Mutex<Self>> {
        let home_dir = env::var("HOME").unwrap_or_else(|_| "/home/restful_otaku".to_string());
        let music_dir = format!("{}/Music", home_dir);

        Arc::new(Mutex::new(App {
            should_quit: false,
            sink: s,
            songs_list: ContentList::from_dir(&music_dir),
            play_deque: VecDeque::new(),
            now_playing: String::new(),
            error_message: None,
        }))
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn pop_play_deque(&mut self) {
        if !self.play_deque.is_empty() {
            let _ = self.play_deque.pop_front();
        }
    }

    pub fn add_play_deque(&mut self, s: String) {
        self.play_deque.push_back(s);
    }
}
