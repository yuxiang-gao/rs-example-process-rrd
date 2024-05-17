//! Log a scalar over time.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_scalar_multiple_plots")
        .save("data.rrd")?;

    for t in 0..((std::f32::consts::TAU * 2.0 * 100.0) as i64) {
        rec.set_time_sequence("step", t);

        // Log two time series under a shared root so that they show in the same plot by default.
        rec.log("trig/sin", &rerun::Scalar::new((t as f64 / 100.0).sin()))?;
        rec.log("trig/cos", &rerun::Scalar::new((t as f64 / 100.0).cos()))?;
    }

    Ok(())
}
