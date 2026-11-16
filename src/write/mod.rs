pub mod options;
pub mod paths;

use std::fs::File;
use zip::ZipWriter;

pub fn write_dir_layer(zip: &mut ZipWriter<File>, options: options::ZipBombOptions) {
    
}