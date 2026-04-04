use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let ambient_dir = manifest_dir.join("ambient");

    println!("cargo:rerun-if-changed={}", ambient_dir.display());

    let mut files = fs::read_dir(&ambient_dir)
        .expect("read ambient dir")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("i"))
        .collect::<Vec<_>>();
    files.sort();

    let mut generated =
        String::from("pub const EMBEDDED_AMBIENT_FILES: &[EmbeddedAmbientFile] = &[\n");
    for path in files {
        let relative = path
            .strip_prefix(&manifest_dir)
            .expect("ambient path within manifest dir");
        generated.push_str(&format!(
            "    EmbeddedAmbientFile {{ relative_path: {:?}, contents: include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\")) }},\n",
            relative.to_string_lossy(),
            normalize_for_include(relative)
        ));
    }
    generated.push_str("];\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    fs::write(out_dir.join("embedded_ambient_generated.rs"), generated)
        .expect("write embedded ambient manifest");
}

fn normalize_for_include(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
