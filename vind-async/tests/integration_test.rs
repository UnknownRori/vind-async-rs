use unknownrori_simple_thread_pool::crossbeam_channel::unbounded;

#[test]
fn create_instance() {
    let mut rt = vind_async::Runtime::new();
    let (task_sender, ready_queue) = unbounded();

    let sender1 = task_sender.clone();
    rt.spawn(async move {
        sender1.clone().send(1).unwrap();
    });

    let sender2 = task_sender.clone();
    rt.spawn(async move {
        let mut _iteration = 0;
        for _ in 0..10_000 {
            _iteration += 1;
        }

        sender2.clone().send(2).unwrap();
    });

    let sender3 = task_sender.clone();
    rt.spawn(async move {
        sender3.clone().send(3).unwrap();
    });

    drop(task_sender);
    rt.run();

    let collection = ready_queue.iter().collect::<Vec<_>>();
    dbg!(&collection);
    let mut collect_iter = collection.iter();
    assert_eq!(collect_iter.next(), Some(&1));
    assert_eq!(collect_iter.next(), Some(&3));
    assert_eq!(collect_iter.next(), Some(&2));
    assert_eq!(collect_iter.next(), None);
}
