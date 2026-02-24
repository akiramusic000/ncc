use std::{ffi::OsStr, hint::spin_loop, path::Path, process::Command};

use notify::{
    EventKind, RecursiveMode, Watcher,
    event::{DataChange, ModifyKind},
};

fn main() {
    let mut watcher = notify::recommended_watcher(|s: Result<notify::Event, notify::Error>| {
        let event = s.expect("error!");
        if !matches!(
            event.kind,
            EventKind::Modify(ModifyKind::Data(DataChange::Content))
        ) {
            return;
        }

        let [path] = &event.paths[..] else { return };
        if path.extension() != Some(OsStr::new("nc")) {
            return;
        }

        println!("Update! {}", path.display());

        let mut child = Command::new("ncc")
            .arg(path.display().to_string())
            .arg("-fsyntax-only")
            .spawn()
            .expect("error!");
        child.wait().expect("error!");

        println!("Build done! {}", path.display());
    })
    .expect("error!");

    watcher
        .watch(Path::new("."), RecursiveMode::Recursive)
        .expect("error!");

    loop {
        spin_loop();
    }
}
