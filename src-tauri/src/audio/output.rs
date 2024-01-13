use anyhow::anyhow;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rtrb::{Producer, RingBuffer};
use std::{thread, time::Duration};
use tokio::task::spawn_blocking;

pub struct AudioOutput {
    pub buf_w: Producer<f32>,
    pub config: cpal::StreamConfig,
}

impl AudioOutput {
    pub fn try_new() -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();

        let device = match host.default_output_device() {
            Some(device) => device,
            _ => return Err(anyhow!("default audio output device not found")),
        };

        let config = match device.default_output_config() {
            Ok(config) => config,
            Err(err) => return Err(anyhow!(err)),
        };

        // Log the output device's name and config
        info!(
            "output device: {}",
            device.name().unwrap_or("Unknown".to_string())
        );
        trace!("output channels: {}", config.channels());
        trace!("sample format: {}", config.sample_format());
        trace!("sample rate: {}", config.sample_rate().0);

        // TODO: support other formats and sample rates
        // Use F32 format for now
        if config.sample_format() != cpal::SampleFormat::F32 {
            return Err(anyhow!("unsupported sample format"));
        }

        let buf_size = ((50 * config.sample_rate().0 as usize) / 1000) * config.channels() as usize;
        let (buf_w, mut buf_r) = RingBuffer::<f32>::new(buf_size);

        let moved_config = config.clone();
        spawn_blocking(move || {
            let stream_result = device.build_output_stream(
                &moved_config.config(),
                move |buf: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    match super::utils::read_from_consumer(&mut buf_r, buf) {
                        Ok(write_len) => buf[write_len..].fill(0.0),
                        Err(_) => {
                            // TODO
                        }
                    }
                },
                move |err| warn!("audio output error: {}", err),
                None,
            );

            let stream = match stream_result {
                Ok(stream) => stream,
                Err(err) => return Err(anyhow!(err)),
            };

            if let Err(err) = stream.play() {
                return Err(anyhow!(err));
            }

            // TODO: this is a hack to keep the audio output stream alive
            thread::sleep(Duration::from_secs(999));

            Ok(())
        });

        Ok(Self {
            buf_w,
            config: config.config(),
        })
    }
}
