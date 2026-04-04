pub struct EmbeddedAmbientFile {
    pub relative_path: &'static str,
    pub contents: &'static str,
}

include!(concat!(env!("OUT_DIR"), "/embedded_ambient_generated.rs"));
