use std::path::{Path, PathBuf};
use std::fs;
use std::io;

#[tokio::main]
async fn main()  {
    let source_dir: &str = "D:\\Downloads";    
    
    if let Err(err) = organise_files(source_dir) {
        eprintln!("Error: {}", err);
    }
    
}
fn make_unique(dest: &str, name: &str) -> String {
    let (filename, extension) = match Path::new(name).file_stem().and_then(|stem| stem.to_str()) {
        Some(filename) => {
            let extension = Path::new(name).extension().and_then(|ext| ext.to_str()).unwrap_or("");
            (filename.to_string(), extension.to_string())
        }
        None => (name.to_string(), String::new()),
    };

    let mut counter: i32 = 1;
    let mut unique_name: PathBuf = PathBuf::from(dest).join(name);

    while unique_name.exists() {
        let new_name = format!("{}({}).{}", filename, counter, extension);
        unique_name = PathBuf::from(dest).join(new_name);
        counter += 1;
    }

    return unique_name.to_string_lossy().to_string().replace("\\", "/")
}

fn organise_files(source: &str) -> io::Result<()> {
    let dest_dir_image: &str = "D:\\Downloads\\Image";
    let dest_dir_audio: &str = "D:\\Downloads\\Audio";

    let image_extensions: [&str;37] = ["jpg", "jpeg", "jpe", "jif", "jfif", "jfi", "png", "gif", "webp", "tiff",
                    "tif", "psd", "raw", "arw", "cr2", "nrw", "k25", "bmp", "dib", "heif", "heic",
                    "ind", "indd", "indt", "jp2", "j2k", "jpf", "jpf", "jpx", "jpm", "mj2", "svg",
                    "svgz", "ai", "eps", "ico", "avif"];
    let audio_extensions: [&str;6] = [".m4a", ".flac", "mp3", ".wav", ".wma", ".aac"];

    // Use read_dir to get an iterator of DirEntry objects
    let entries = fs::read_dir(source)?;

    for entry in entries {
        let entry = entry?;
        
        // Check if the entry is a file
        if entry.file_type()?.is_file() {
            if let Some(extension) = entry.path().extension() {
                if image_extensions.contains(&extension.to_str().unwrap()) {
                    let unique_dest: String = make_unique(dest_dir_image, entry.file_name().to_str().unwrap());
                    let destination_path: &Path = Path::new(&unique_dest);// Use Path::new and join for path manipulation
                    fs::rename(entry.path(), &destination_path)?; // Use rename for moving files
                    println!("Moved {} to {}", entry.path().display(), destination_path.display());
                }
                if audio_extensions.contains(&extension.to_str().unwrap()) {
                    let unique_dest: String = make_unique(dest_dir_audio, entry.file_name().to_str().unwrap());
                    let destination_path: &Path = Path::new(&unique_dest);// Use Path::new and join for path manipulation
                    fs::rename(entry.path(), &destination_path)?; // Use rename for moving files
                    println!("Moved {} to {}", entry.path().display(), destination_path.display());
                }
            }
        }
    }

    Ok(())
}