use anyhow::anyhow;
use std::fs::File;
use symphonia::{
    core::{
        audio::SampleBuffer,
        codecs::{DecoderOptions, CODEC_TYPE_NULL},
        errors::Error,
        formats::FormatOptions,
        io::{MediaSource, MediaSourceStream},
        meta::MetadataOptions,
        probe::Hint,
    },
    default::get_codecs,
};

pub async fn decode_mp3(path: &str, buf: &mut Vec<f32>) -> anyhow::Result<()> {
    // Open the media source.
    let src = File::open(path).expect("failed to open media");

    trace!("src: {:?}", src.byte_len());

    // Create the media source stream.
    let mss = MediaSourceStream::new(Box::new(src), Default::default());

    // Create a probe hint using the file extension.
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    // Use the default options for metadata and format readers.
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    // Probe the media source.
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    // Get the instanteated format reader.
    let mut format = probed.format;

    // Find the first audio track with a known (decodable) codec.
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("no supported audio tracks found");

    // Use the default options for the decoder.
    let dec_opts: DecoderOptions = DecoderOptions {
        verify: false,
        ..Default::default()
    };

    // Create the decoder.
    let mut decoder = get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("unsupported codec");

    let track_id = track.id;

    let mut run_once = false;

    // TODO: blocks until all is decoded
    loop {
        // TODO: consider 3 consecutive errors as fatal
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                unimplemented!("the only usage of this is for chained OGG physical streams.")
            }
            Err(err) => {
                break Err(anyhow!(err));
            }
        };

        while !format.metadata().is_latest() {
            format.metadata().pop();
        }

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                if !run_once {
                    run_once = true;
                    trace!("sample rate: {}", decoded.spec().rate);
                    trace!("number of channels: {}", decoded.spec().channels.count());
                }

                // The packet was decoded successfully.
                let mut sample: SampleBuffer<f32> =
                    SampleBuffer::new(decoded.capacity() as u64, *decoded.spec());

                sample.copy_interleaved_ref(decoded);
                buf.append(&mut sample.samples().to_vec());

                // let mut converted = decoded.make_equivalent::<f32>();
                // decoded.convert(&mut converted);

                // let mut sample: SampleBuffer<f32> =
                //     SampleBuffer::new(converted.capacity() as u64, *converted.spec());

                // sample.copy_interleaved_typed::<f32>(&converted);
                // buf.append(&mut sample.samples().to_vec());
            }
            Err(Error::IoError(_)) => {
                // The packet failed to decode due to an IO error. Skip it.
                continue;
            }
            Err(Error::DecodeError(_)) => {
                // The packet failed to decode to due a decoding error. Skip it.
                continue;
            }
            Err(err) => {
                error!("{}", err);
                continue;
            }
        }
    }
}
