use std::sync::mpsc::Receiver;
use core_foundation::error::*;
use core_media_rs::cm_sample_buffer::CMSampleBuffer;

trait SysAudioStream {
    fn start(&mut self) -> Result<Receiver<CMSampleBuffer>, CFError>;
    fn stop(&mut self) -> Result<(), CFError>;
}