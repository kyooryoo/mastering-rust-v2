use::std::env;
use::std::path::Path;

fn main() {
    let image_path = env::args().skip(1).next().unwrap();
    // second command argument defines the degree to rotate, default to 90
    let rotate_method = env::args().skip(2).next().unwrap_or("90".to_string());
    // third argument defines if rotate inplace or to new file, default to new
    let inplace = env::args().skip(3).next().unwrap_or("false".to_string());

    // prepare new file path and new file name
    let new_image_path = &image_path.replace(".", "_rotated.");
    let new_path = Path::new(&new_image_path);

    let path = Path::new(&image_path);
    let img = image::open(path).unwrap();
    // perform rotation per input rotation method
    let rotated = match &rotate_method as &str {
        "90" => img.rotate90(),
        "180" => img.rotate180(),
        "270" => img.rotate270(),
        _ => {
            println!("please provide rotate method: 90 180 270");
            img
        }
    };
    // perform inplace rotation or not per inplace indicator
    match &inplace as &str {
        "true" => rotated.save(path).unwrap(),
        _ => rotated.save(new_path).unwrap()
    }
}
