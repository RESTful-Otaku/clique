use crate::app::App;
use crate::list::ContentList;
use rodio::Decoder;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn play_sound(app: &mut App) {
    if !app.sink.is_paused() && app.sink.empty() && !app.play_deque.is_empty() {
        let path = app.play_deque.front().unwrap();
        app.now_playing = path.clone().split("/").last().unwrap().to_string();

        let opened = File::open(path).unwrap();
        let file = BufReader::new(opened);
        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();

        app.sink.append(source);
        app.pop_play_deque();
    }
}

fn append_sound(index: u32, app: &mut App) {
    let mut paths = fs::read_dir(app.songs_list.path.clone()).unwrap();

    let path = format!(
        "{}",
        paths.nth(index as usize).unwrap().unwrap().path().display()
    );

    // Check if this is a folder
    if Path::new(&path).is_dir() {
        // Go inside
        app.songs_list = ContentList::from_dir(&path);
    } else {
        app.add_play_deque(path);
    }
}

pub fn update(app: &mut App, key_event: crossterm::event::KeyEvent) {
    match key_event.code {
        crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Char('q') => app.quit(),
        crossterm::event::KeyCode::Char('c') | crossterm::event::KeyCode::Char('C') => {
            if key_event.modifiers == crossterm::event::KeyModifiers::CONTROL {
                app.quit()
            }
        }
        crossterm::event::KeyCode::Char(c) => {
            match c {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    append_sound(c.to_digit(10).unwrap(), app);
                }
                ' ' => {
                    if app.sink.is_paused() {
                        app.sink.play();
                    } else {
                        app.sink.pause();
                    }
                }
                '+' => {
                    if app.sink.volume() <= 2.0 {
                        app.sink.set_volume(app.sink.volume() + 0.05);
                    }
                }
                '-' => {
                    if app.sink.volume() - 0.05 >= 0.0 {
                        app.sink.set_volume(app.sink.volume() - 0.05);
                    } else {
                        app.sink.set_volume(0.0);
                    }
                }
                'r' => {
                    app.pop_play_deque();
                }
                's' => {
                    app.sink.clear();
                    app.sink.play();
                }
                _ => {}
            };
        }
        crossterm::event::KeyCode::Up => {
            app.songs_list.prev();
        }
        crossterm::event::KeyCode::Down => {
            app.songs_list.next();
        }
        crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Right => {
            append_sound(app.songs_list.index as u32, app);
        }
        crossterm::event::KeyCode::Left => {
            let new_path: Vec<&str> = app.songs_list.path.split("/").collect();
            let sum_path = &new_path[..new_path.len() - 1].join("/");

            app.songs_list = ContentList::from_dir(sum_path);
        }

        _ => {}
    };

    play_sound(app);
}
