
use std::sync::Arc;

use sdl2_mixer as mix;
use current::Current;
use dyon::*;

pub type MusicTracks = Vec<(Arc<String>, mix::Music)>;

pub fn register_sound(module: &mut Module) {
    module.add(Arc::new("load__music".into()), load__music, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::Text],
        ret: Type::Result(Box::new(Type::Text))
    });
    module.add(Arc::new("music".into()), music, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::Text],
        ret: Type::Option(Box::new(Type::F64))
    });
    module.add(Arc::new("play_forever__music".into()), play_forever__music, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::F64],
        ret: Type::Void
    });
}

dyon_fn!{fn load__music(file: Arc<String>) -> Result<Arc<String>, String> {
    use std::path::Path;

    let music_tracks = unsafe { &mut *Current::<MusicTracks>::new() };

    let track = {
        let path = Path::new(&**file);
        try!(mix::Music::from_file(&path))
    };
    music_tracks.push((file.clone(), track));

    Ok(file)
}}

dyon_fn!{fn music(name: Arc<String>) -> Option<usize> {
    let music_tracks = unsafe { &*Current::<MusicTracks>::new() };

    for (i, &(ref track, _)) in music_tracks.iter().enumerate() {
        if track == &name { return Some(i); }
    }
    None
}}

dyon_fn!{fn play_forever__music(ind: usize) {
    let music_tracks = unsafe { &*Current::<MusicTracks>::new() };

    let _ = music_tracks[ind].1.play(-1);
}}
