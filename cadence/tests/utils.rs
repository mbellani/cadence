use cadence::prelude::*;
use cadence::StatsdClient;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn run_arc_threaded_test(client: StatsdClient, num_threads: u64, iterations: u64) {
    let shared_client = Arc::new(client);

    let threads: Vec<_> = (0..num_threads)
        .map(|_| {
            let local_client = shared_client.clone();

            thread::spawn(move || {
                for i in 0..iterations {
                    local_client.count("some.counter", i as i64).unwrap();
                    local_client.time("some.timer", i).unwrap();
                    local_client.time("some.timer", Duration::from_millis(i)).unwrap();
                    local_client.gauge("some.gauge", i).unwrap();
                    local_client.gauge("some.gauge", i as f64).unwrap();
                    local_client.meter("some.meter", i).unwrap();
                    local_client.histogram("some.histogram", i).unwrap();
                    local_client
                        .histogram("some.histogram", Duration::from_nanos(i))
                        .unwrap();
                    local_client.histogram("some.histogram", i as f64).unwrap();
                    local_client.distribution("some.distribution", i).unwrap();
                    local_client.distribution("some.distribution", i as f64).unwrap();
                    local_client.set("some.set", i as i64).unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
}
