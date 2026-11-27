pub fn size_to_string_candy(val: usize) -> String {
    const EXTENSIONS: [&str; 5] = ["b", "kb", "mb", "gb", "tb"];
    let mut size = val as f32;
    let mut i = 0;
    while size > 1024.0 && i < EXTENSIONS.len() - 1 {
        size = size / 1024.0;
        i += 1;
    }
    format!("{:.2} {}", size as f32, EXTENSIONS[i])
}
