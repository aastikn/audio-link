use rodio::{OutputStream, Sink, Source};
use std::time::Duration;
use std::sync::Arc;
use std::error::Error;

struct CapturedAudioSource {
    sample_rate: u32,
    channels: u16,
    data: Arc<Vec<f32>>,
    position: usize,
}

impl CapturedAudioSource {
    fn new(sample_rate: u32, channels: u16, data: Vec<f32>) -> Self {
        Self {
            sample_rate,
            channels,
            data: Arc::new(data),
            position: 0,
        }
    }
}

impl Iterator for CapturedAudioSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.data.len() {
            let sample = self.data[self.position];
            self.position += 1;
            Some(sample)
        } else {
            None
        }
    }
}

impl Source for CapturedAudioSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.data.len() - self.position)
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_secs_f32(
            self.data.len() as f32 / self.sample_rate as f32 / self.channels as f32,
        ))
    }
}

pub fn play_captured_audio(audio_data: Vec<f32>, sample_rate: u32, channels: u16) -> Result<(), Box<dyn Error>> {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()?;
    
    // Create a sink
    let sink = Sink::try_new(&stream_handle)?;

    // Create our custom source
    let source = CapturedAudioSource::new(sample_rate, channels, audio_data);
    
    // Add the source to the sink
    sink.append(source);

    // Play the sound
    sink.play();

    // Calculate the duration of the audio
    let duration = Duration::from_secs_f32(audio_data.len() as f32 / sample_rate as f32 / channels as f32);

    // Wait for the audio to finish playing
    std::thread::sleep(duration);

    Ok(())
}
