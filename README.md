# Fluxion

Simple async library for data stream processing

Example usage (with kafka feature):
```
    let consumer = KafkaConsumer::new(
        "localhost",
        9092,
        "input-test-topic",
        "example-consumer-group",
        "latest",
    ).expect("Error creating consumer");
    let producer = KafkaProducer::new("localhost", 9092, "output-test-topic")
      .expect("Error creating producer");

    let mut pipeline = Pipeline::<String>::new();
    pipeline.add_operator(fluxion::operators::Filter::new(|data: &String| {
        data.contains("Kafka")
    }));
    let _ = run_pipeline(Arc::new(consumer), Arc::new(producer), pipeline, 100).await;
```
