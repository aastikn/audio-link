mod audio_stream_handler{
    pub mod stream_getter;
    pub mod audio_player;
}


use audio_stream_handler::{
    stream_getter,
    audio_player
};
        

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the audio data
    let (audio_data, sample_rate, channels) =stream_getter::get_stream()?;

    // Play the captured audio
    audio_player::play_captured_audio(audio_data, sample_rate, channels)?;


    Ok(())
}
