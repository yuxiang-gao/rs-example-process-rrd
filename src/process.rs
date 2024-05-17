use re_data_store::{RangeQuery, ResolvedTimeRange};
use re_entity_db::StoreBundle;
use re_query::PromiseResolver;
use re_types::components::Scalar;
use rerun::{Loggable, Timeline};

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let Some(rrd_path) = args.get(1) else {
        eprintln!("Usage: {} <rrd-file>", args[0]);
        std::process::exit(1);
    };

    let encoded = std::fs::File::open(rrd_path).unwrap();
    let bundle =
        StoreBundle::from_rrd(re_log_encoding::decoder::VersionPolicy::Warn, encoded).unwrap();

    for rrd in bundle.entity_dbs() {
        println!("Found {} rows", rrd.num_rows());

        let entities = rrd.entity_paths();
        println!("Containing {} entities", entities.len());
        for entity in rrd.entity_paths() {
            println!("- {:?}", entity);
        }

        let timelines: Vec<_> = rrd.timelines().collect();
        println!("Containing {} timelines", timelines.len());
        for timeline in &timelines {
            println!("- {:?}", timeline);
        }

        for entity in entities {
            let range_query = RangeQuery::new(
                Timeline::new_sequence("step"),
                ResolvedTimeRange::EVERYTHING,
            );

            let resolver = PromiseResolver {};

            if let Some(range_scalar) = rrd
                .query_caches()
                .range(rrd.data_store(), &range_query, entity, [Scalar::name()])
                .get(Scalar::name())
                .map(|results| results.to_dense::<Scalar>(&resolver))
            {
                for ((time, _), scalars) in range_scalar.range_indexed().take(5) {
                    if let Some(scalar) = scalars.first() {
                        println!(
                            "Scalar value of {:?} is {:?} @ Timestep: {:?}",
                            entity,
                            scalar.0,
                            time.as_i64()
                        );
                    }
                }
            }
        }
    }
}
