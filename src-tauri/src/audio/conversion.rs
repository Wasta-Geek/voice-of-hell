use anyhow::{Context, Result};
use rubato::{
    Fft, FixedSync, Resampler, audioadapter::Adapter, audioadapter_buffers::owned::InterleavedOwned,
};

/// Resampler chunk size in frames (only impacts processing speed / quality, not the result length)
const RESAMPLER_CHUNK_SIZE: usize = 1024;

/// Convert interleaved samples from a channel count to another one
/// - same count: unchanged
/// - from mono: the channel is copied to every output channel
/// - to mono: channels are averaged
/// - otherwise: first channels are kept (front left / right in standard layouts), missing ones are silent
pub fn convert_channels(samples: Vec<f32>, from_channels: usize, to_channels: usize) -> Vec<f32> {
    if from_channels == to_channels {
        return samples;
    }

    samples
        .chunks_exact(from_channels)
        .flat_map(|frame| {
            (0..to_channels).map(move |channel| match (from_channels, to_channels) {
                (1, _) => frame[0],
                (_, 1) => frame.iter().sum::<f32>() / from_channels as f32,
                _ => frame.get(channel).copied().unwrap_or(0.0),
            })
        })
        .collect()
}

/// Resample interleaved samples from a sample rate to another one
pub fn resample(
    samples: Vec<f32>,
    channels: usize,
    from_sample_rate: u32,
    to_sample_rate: u32,
) -> Result<Vec<f32>> {
    if from_sample_rate == to_sample_rate || samples.is_empty() {
        return Ok(samples);
    }

    let frames = samples.len() / channels;
    let input = InterleavedOwned::new_from(samples, channels, frames)
        .with_context(|| "Invalid samples length for resampling")?;

    let mut resampler = Fft::<f32>::new(
        from_sample_rate as usize,
        to_sample_rate as usize,
        RESAMPLER_CHUNK_SIZE,
        channels,
        FixedSync::Both,
    )
    .with_context(|| {
        format!("Couldn't create resampler from {from_sample_rate} Hz to {to_sample_rate} Hz")
    })?;

    // Processes the whole clip, resampler delay is trimmed
    let output = resampler
        .process_all(&input, input.frames(), None)
        .with_context(|| "Resampling failed")?;

    Ok(output.take_data())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_channels_mono_to_stereo_duplicates() {
        assert_eq!(
            convert_channels(vec![0.1, 0.2], 1, 2),
            vec![0.1, 0.1, 0.2, 0.2]
        );
    }

    #[test]
    fn convert_channels_stereo_to_mono_averages() {
        assert_eq!(
            convert_channels(vec![0.2, 0.4, -1.0, 1.0], 2, 1),
            vec![0.3, 0.0]
        );
    }

    #[test]
    fn convert_channels_keeps_first_channels() {
        // 3 channels -> stereo: third channel dropped
        assert_eq!(
            convert_channels(vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6], 3, 2),
            vec![0.1, 0.2, 0.4, 0.5]
        );
        // Stereo -> 3 channels: third channel silent
        assert_eq!(convert_channels(vec![0.1, 0.2], 2, 3), vec![0.1, 0.2, 0.0]);
    }

    #[test]
    fn resample_keeps_duration_and_pitch() {
        // 1 second stereo 440 Hz sine at 48 kHz
        let frequency = 440.0;
        let samples: Vec<f32> = (0..48_000)
            .flat_map(|frame| {
                let value =
                    (2.0 * std::f32::consts::PI * frequency * frame as f32 / 48_000.0).sin();
                [value, value]
            })
            .collect();

        let resampled = resample(samples, 2, 48_000, 44_100).unwrap();

        // Still 1 second long at 44.1 kHz (±1 frame)
        let frames = resampled.len() / 2;
        assert!(frames.abs_diff(44_100) <= 1, "got {frames} frames");

        // Same pitch: count rising zero crossings of the left channel (≈ 440 per second)
        let left: Vec<f32> = resampled.iter().step_by(2).copied().collect();
        let crossings = left
            .windows(2)
            .filter(|pair| pair[0] < 0.0 && pair[1] >= 0.0)
            .count();
        assert!(crossings.abs_diff(440) <= 2, "got {crossings} crossings");
    }

    #[test]
    fn resample_same_rate_is_unchanged() {
        let samples = vec![0.1, 0.2, 0.3, 0.4];
        assert_eq!(
            resample(samples.clone(), 2, 44_100, 44_100).unwrap(),
            samples
        );
    }
}
