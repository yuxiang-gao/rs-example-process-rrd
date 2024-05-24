//! Log a scalar over time.
use ndarray::{s, Array, ShapeBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar_multiple_plots")
        .save("data.rrd")?;

    for t in 0..((std::f32::consts::TAU * 2.0 * 100.0) as i64) {
        rec.set_time_sequence("step", t);

        let mut image = Array::<u16, _>::from_elem((200, 300).f(), 65535);
        image
            .slice_mut(s![50..150, 50..150])
            .fill((20000.0 * ((t as f64) / 100.0).sin()) as u16);
        image
            .slice_mut(s![130..180, 100..280])
            .fill((45000.0 * ((t as f64) / 100.0).sin()) as u16);

        let depth_image = rerun::DepthImage::try_from(image.clone())?.with_meter(10000.0);

        // If we log a pinhole camera model, the depth gets automatically back-projected to 3D
        rec.log(
            "world/camera",
            &rerun::Pinhole::from_focal_length_and_resolution(
                [200.0, 200.0],
                [image.shape()[1] as f32, image.shape()[0] as f32],
            ),
        )?;

        // Log two time series under a shared root so that they show in the same plot by default.
        rec.log("trig/sin", &rerun::Scalar::new((t as f64 / 100.0).sin()))?;
        rec.log("trig/cos", &rerun::Scalar::new((t as f64 / 100.0).cos()))?;
        rec.log("world/camera/depth", &depth_image)?;
    }

    Ok(())
}
