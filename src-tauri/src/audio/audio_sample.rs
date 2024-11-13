use std::default;

pub struct AudioSample<T> {
    samples: Vec<T>,
    is_mono: bool,
    sample_rate: u32,
}
impl<T> AudioSample<T>
where
    T: Copy,
{
    pub fn new(samples: Vec<T>, is_mono: bool, sample_rate: u32) -> Self {
        Self {
            samples,
            is_mono,
            sample_rate,
        }
    }
    pub fn get_sample(&self, left_index: usize) -> Option<(T, T)> {
        let left_sample = *self.samples.get(left_index)?;
        let right_sample = if self.is_mono {
            left_sample
        } else {
            *self.samples.get(left_index + 1)?
        };
        Some((left_sample, right_sample))
    }
    pub fn set_samples(&mut self, new_sample: Vec<T>) {
        self.samples = new_sample;
    }
    pub fn get_sample_size(&self) -> usize {
        self.samples.len()
    }
}

pub fn load_wav(file_path: &str) -> Result<AudioSample<f32>, hound::Error> {
    let reader = hound::WavReader::open(file_path)?;
    let spec = reader.spec();
    println!("WAV Spec: {:?}", spec);

    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader
            .into_samples::<f32>()
            .collect::<Result<Vec<f32>, _>>()?,
        hound::SampleFormat::Int => reader
            .into_samples::<i32>()
            .map(|s| s.map(|sample| sample as f32 / i32::MAX as f32))
            .collect::<Result<Vec<f32>, _>>()?,
    };

    Ok(AudioSample {
        samples,
        sample_rate: spec.sample_rate,
        is_mono: spec.channels == 1,
    })
}
