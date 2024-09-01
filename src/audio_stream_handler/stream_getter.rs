use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal:: SampleFormat;
use std::sync::{Arc, Mutex};

pub fn get_stream() -> Result<(), Box<dyn std::error::Error>> {
    // Get the default host
    let host = cpal::default_host();

    // Get the default input device
    let device = host.default_input_device()
        .expect("No input device available");

    println!("Using input device: '{}'", device.name()?);

    // Get the default input config
    let config = device.default_input_config()?;
    println!("Default input config: {:?}", config);

    // Create a vector to store some samples
    let samples = Arc::new(Mutex::new(Vec::new()));
    let samples_clone = samples.clone();

    // Build the input stream
    let stream = match config.sample_format() {
        SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                samples_clone.lock().unwrap().extend_from_slice(data);
            },
            err_fn,
            None
        )?,
        SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data: &[i16], _: &_| {
                samples_clone.lock().unwrap().extend(data.iter().map(|&s| s as f32 / i16::MAX as f32));
            },
            err_fn,
            None
        )?,
        SampleFormat::U16 => device.build_input_stream(
            &config.into(),
            move |data: &[u16], _: &_| {
                samples_clone.lock().unwrap().extend(data.iter().map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0));
            },
            err_fn,
            None
        )?,
        _ => return Err("Unsupported sample format".into()),
    };

    // Play the stream
    stream.play()?;

    // Record for 3 seconds
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Stop the stream
    drop(stream);

    // Analyze the recorded samples
    let recorded_samples = samples.lock().unwrap();
    println!("Recorded {} samples", recorded_samples.len());

    if !recorded_samples.is_empty() {
        let max = recorded_samples.iter().fold(0.0f32, |a, &b| a.max(b.abs()));
        println!("Max amplitude: {}", max);
    }

    Ok(())
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("An error occurred on stream: {}", err);
}
