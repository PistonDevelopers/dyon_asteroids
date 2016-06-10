
use std::sync::Arc;

use sdl2_mixer as mix;
use current::Current;
use dyon::*;

pub type MusicTracks = Vec<(Arc<String>, mix::Music)>;
pub type SoundTracks = Vec<(Arc<String>, mix::Chunk)>;

pub fn register_sound(module: &mut Module) {
    module.add(Arc::new("load__music".into()), load__music, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::Text],
        ret: Type::Result(Box::new(Type::Text))
    });
    module.add(Arc::new("load__sound".into()), load__sound, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::Text],
        ret: Type::Result(Box::new(Type::Text))
    });
    module.add(Arc::new("music".into()), music, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::Text],
        ret: Type::Option(Box::new(Type::F64))
    });
    module.add(Arc::new("sound".into()), sound, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::Text],
        ret: Type::Option(Box::new(Type::F64))
    });
    module.add(Arc::new("play_forever__music".into()), play_forever__music, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::F64],
        ret: Type::Void
    });
    module.add(Arc::new("play_once__music".into()), play_once__music, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::F64],
        ret: Type::Void
    });
    module.add(Arc::new("play_once__sound".into()), play_once__sound, PreludeFunction {
        lts: vec![Lt::Default],
        tys: vec![Type::F64],
        ret: Type::Void
    });
    module.add(Arc::new("set__sound_volume".into()), set__sound_volume, PreludeFunction {
        lts: vec![Lt::Default; 2],
        tys: vec![Type::F64; 2],
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

dyon_fn!{fn load__sound(file: Arc<String>) -> Result<Arc<String>, String> {
    use std::path::Path;

    let sound_tracks = unsafe { &mut *Current::<SoundTracks>::new() };

    let track = {
        let path = Path::new(&**file);
        try!(mix::Chunk::from_file(&path))
    };
    sound_tracks.push((file.clone(), track));

    Ok(file)
}}

dyon_fn!{fn music(name: Arc<String>) -> Option<usize> {
    let music_tracks = unsafe { &*Current::<MusicTracks>::new() };

    for (i, &(ref track, _)) in music_tracks.iter().enumerate() {
        if track == &name { return Some(i); }
    }
    None
}}

dyon_fn!{fn sound(name: Arc<String>) -> Option<usize> {
    let sound_tracks = unsafe { &*Current::<SoundTracks>::new() };

    for (i, &(ref track, _)) in sound_tracks.iter().enumerate() {
        if track == &name { return Some(i); }
    }
    None
}}

dyon_fn!{fn play_forever__music(ind: usize) {
    let music_tracks = unsafe { &*Current::<MusicTracks>::new() };

    let _ = music_tracks[ind].1.play(-1);
}}

dyon_fn!{fn play_once__music(ind: usize) {
    let music_tracks = unsafe { &*Current::<MusicTracks>::new() };

    let _ = music_tracks[ind].1.play(0);
}}

dyon_fn!{fn play_once__sound(ind: usize) {
    let sound_tracks = unsafe { &*Current::<SoundTracks>::new() };

    let _ = mix::Channel::all().play(&sound_tracks[ind].1, 0);
}}

dyon_fn!{fn set__sound_volume(ind: usize, volume: f64) {
    let sound_tracks = unsafe { &mut *Current::<SoundTracks>::new() };

    sound_tracks[ind].1.set_volume(volume as isize);
}}
