mod audio_stream_handler{
    pub mod stream_getter;
}


use audio_stream_handler::stream_getter;

    
fn main() {
    stream_getter::get_stream();
    println!("Hello, world!");
}
