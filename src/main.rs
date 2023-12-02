use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rtow::ray::Ray;
use rtow::vec::Vec3;

type Point3 = Vec3;
type Color = Vec3;

fn main() {
    // Image

    let image_width = 256;
    let image_height = 256;

    // Progress bar

    let pb = ProgressBar::new(image_height as u64 * image_width as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap(),
    );

    // Render

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width as f64 - 1.0),
                j as f64 / (image_height as f64 - 1.0),
                0.0,
            );
            pixel_color.write();
            pb.inc(1);
        }
        print!("\n");
    }

    pb.finish();
}
