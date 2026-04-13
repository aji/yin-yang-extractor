pub fn standard_normal(x: f32) -> f32 {
    (-0.5 * x * x).exp() / (std::f32::consts::TAU).sqrt()
}

pub struct Array {
    pub data: Vec<f32>,
    pub mean: f32,
    pub var: f32,
}

impl Array {
    pub fn new(data: Vec<f32>) -> Array {
        let mean = data.iter().sum::<f32>() / data.len() as f32;
        let var = data.iter().map(|x| (*x - mean).powi(2)).sum::<f32>() / data.len() as f32;
        Array { data, mean, var }
    }
}

pub struct HoldSampler<'a, T> {
    inner: &'a [T],
}

impl<'a, T> HoldSampler<'a, T> {
    pub fn new(inner: &'a [T]) -> HoldSampler<'a, T> {
        HoldSampler { inner }
    }

    pub fn get(&self, idx: isize) -> &'a T {
        let actual_index = idx.max(0).min(self.inner.len() as isize - 1);
        self.inner.get(actual_index as usize).unwrap()
    }
}

impl<'a> HoldSampler<'a, f32> {
    pub fn get_linear(&self, idx: f32) -> f32 {
        let x0 = *self.get(idx.floor() as isize);
        let x1 = *self.get(idx.ceil() as isize);
        let f = idx.fract();
        x0 * (1.0 - f) + x1 * f
    }
}

pub struct ZeroSampler<'a, T> {
    zero: T,
    inner: &'a [T],
}

impl<'a, T> ZeroSampler<'a, T> {
    pub fn new(zero: T, inner: &'a [T]) -> ZeroSampler<'a, T> {
        ZeroSampler { zero, inner }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn get<'s>(&'s self, idx: isize) -> &'a T
    where
        's: 'a,
    {
        if idx < 0 || self.inner.len() as isize <= idx {
            &self.zero
        } else {
            &self.inner[idx as usize]
        }
    }
}

impl<'a> ZeroSampler<'a, f32> {
    pub fn get_linear(&self, idx: f32) -> f32 {
        let x0 = *self.get(idx.floor() as isize);
        let x1 = *self.get(idx.ceil() as isize);
        let f = idx.fract();
        x0 * (1.0 - f) + x1 * f
    }

    pub fn get_linear_grad(&self, idx: f32) -> (f32, f32) {
        let x0 = *self.get(idx.floor() as isize);
        let x1 = *self.get(idx.ceil() as isize);
        let f = idx.fract();
        let m = x1 - x0;
        (x0 - f * m, m)
    }
}

pub fn naive_forward_autocorr(sig: &[f32]) -> Vec<f32> {
    let sampler = ZeroSampler::new(0.0, &sig[..]);
    if sig.len() == 0 {
        panic!("input empty");
    }
    (0..sig.len() as isize)
        .map(|lag| {
            (0..sig.len() as isize)
                .map(|i| sampler.get(i) * sampler.get(i - lag))
                .sum()
        })
        .collect()
}

pub fn argmax(sig: &[f32]) -> usize {
    if sig.len() == 0 {
        panic!("input empty");
    }
    let mut max_i = 0;
    let mut max_x = sig[0];
    for (i, x) in sig.iter().enumerate().skip(1) {
        if *x > max_x {
            max_i = i;
            max_x = *x;
        }
    }
    max_i
}

#[allow(unused)]
pub fn argsort(sig: &[f32]) -> Vec<usize> {
    let mut with_indices: Vec<(usize, f32)> = sig.iter().copied().enumerate().collect();
    with_indices.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
    with_indices.into_iter().map(|(i, _)| i).collect()
}
