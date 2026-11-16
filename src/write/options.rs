use zip::write::SimpleFileOptions;
use crate::write::paths;

pub struct ZipBombOptions {
    zip_options: SimpleFileOptions,
    count: u32,
    naming: paths::naming::Naming,
    root: String,
}

impl ZipBombOptions {

}