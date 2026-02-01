extern crate taglib;

use std::env;

pub fn main() {
    const EMPTY: &str = "";

    let args: Vec<String> = env::args().collect();

    for i in 1..args.len() {
        let ref arg = args[i];
        let file = match taglib::File::new(arg) {
            Ok(f) => f,
            Err(e) => {
                println!("Invalid file {} (error: {:?})", arg, e);
                continue;
            }
        };

        println!("*** \"{}\" ***", arg);

        match file.tag() {
            Ok(t) => {
                println!("-- TAG --");
                println!("title   - {}", t.title().unwrap_or_default());
                println!("artist  - {}", t.artist().unwrap_or_default());
                println!("album   - {}", t.album().unwrap_or_default());
                println!("year    - {}", t.year()
                    .map_or_else(|| EMPTY.to_string(), |t| t.to_string()));
                println!("comment - {}", t.comment().unwrap_or_default());
                println!("track   - {}", t.track()
                    .map_or_else(|| EMPTY.to_string(), |t| t.to_string()));
                println!("genre   - {}", t.genre().unwrap_or_default());

                println!("-- File TAG --");
                println!("album artist    - {}", t.album_artist().unwrap_or_default());
                println!("composer        - {}", t.composer().unwrap_or_default());
                println!("track total     - {}", t.track_total()
                    .map_or_else(|| EMPTY.to_string(), |t| t.to_string()));
                println!("disc number     - {}", t.disc_number()
                    .map_or_else(|| EMPTY.to_string(), |t| t.to_string()));
                println!("disc total      - {}", t.disc_total()
                    .map_or_else(|| EMPTY.to_string(), |t| t.to_string()));

                println!("-- PROPERTY --");
                let result_keys = file.keys();
                if result_keys.is_ok() {
                    let keys = result_keys.unwrap();
                    println!("{} keys.", keys.len());
                    for key in keys {
                        println!("{}: {:?}",
                                 key,
                                 file.get_property(&key).unwrap_or_default());
                    }
                } else {
                    println!("No available properties for {} (error: {:?})",
                             arg,
                             result_keys.err().unwrap());
                }
            }
            Err(e) => {
                println!("No available tags for {} (error: {:?})", arg, e);
            }
        }

        match file.audioproperties() {
            Ok(p) => {
                let secs = p.length() % 60;
                let mins = (p.length() - secs) / 60;

                println!("-- AUDIO --");
                println!("bitrate     - {}", p.bitrate());
                println!("sample rate - {}", p.samplerate());
                println!("channels    - {}", p.channels());
                println!("length      - {}m:{}s", mins, secs);
            }
            Err(e) => {
                println!("No available audio properties for {} (error: {:?})", arg, e);
            }
        }
    }
}
