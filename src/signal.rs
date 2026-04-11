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

pub struct ZeroSampler<'a, T> {
    zero: T,
    inner: &'a [T],
}

impl<'a, T> ZeroSampler<'a, T> {
    pub fn new(zero: T, inner: &'a [T]) -> ZeroSampler<'a, T> {
        ZeroSampler { zero, inner }
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
