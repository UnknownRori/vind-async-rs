#[cfg(test)]
mod test {
    use std::{sync::Arc, thread, time::Duration};
    use vind_mutex::Mutex;

    use rand::Rng;

    #[test]
    fn test_first_case() {
        let shared_counter = Arc::new(Mutex::new(0));
        let mut threads = vec![];

        for _ in 0..10 {
            let b = Arc::clone(&shared_counter);

            threads.push(thread::spawn(move || {
                for _ in 0..100 {
                    let mut unlocked_counter = b.acquire();

                    let mut rng = rand::thread_rng();
                    let time = rng.gen_range(1..100);

                    thread::sleep(Duration::from_micros(time));

                    (*unlocked_counter) += 1;
                }
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        assert_eq!(*shared_counter.acquire(), 1000);
    }
    //
}
