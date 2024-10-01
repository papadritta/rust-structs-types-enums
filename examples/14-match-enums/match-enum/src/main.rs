enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
}

fn format_size(size: f64) -> String {
    let filesize = match size {
        0.0..=999.0 => FileSize::Bytes(size),
        1000.0..=999_999.0 => FileSize::Kilobytes(size / 1000.0),
        1_000_000.0..=999_999_999.0 => FileSize::Megabytes(size / 1_000_000.0),
        1_000_000_000.0..=999_999_999_999.0 => FileSize::Gigabytes(size / 1_000_000_000.0),
        _ => FileSize::Terabytes(size / 1_000_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb),
    }
}

fn main() {
    let result = format_size(2.44);
    println!("{}", result)
}
