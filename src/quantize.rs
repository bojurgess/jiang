#![allow(dead_code)]

use crate::color::Color;

pub enum Algorithm {
    MedianCut,
    Octree,
}

pub fn quantize(pixels: &[Color], algorithm: Algorithm, k: usize) -> Vec<(Color, u32)> {
    match algorithm {
        Algorithm::MedianCut => median_cut::quantize(pixels, k),
        Algorithm::Octree => octree::quantize(pixels, k),
    }
}

pub mod median_cut {
    use crate::color::Color;

    pub fn quantize(pixels: &[Color], k: usize) -> Vec<(Color, u32)> {
        let mut buckets: Vec<Vec<Color>> = vec![pixels.to_vec()];
        while buckets.len() < k {
            let largest = buckets
                .iter()
                .enumerate()
                .max_by_key(|(_, b)| b.len())
                .map(|(i, _)| i)
                .unwrap();
            let bucket = buckets.remove(largest);
            let (a, b) = cut(bucket);
            buckets.push(a);
            buckets.push(b);
        }

        buckets
            .into_iter()
            .filter(|b| !b.is_empty())
            .map(|bucket| {
                let population = bucket.len() as u32;
                let sum = bucket.iter().fold([0u32; 3], |mut acc, c| {
                    acc[0] += c.r as u32;
                    acc[1] += c.g as u32;
                    acc[2] += c.b as u32;
                    acc
                });
                let colour = Color::new(
                    (sum[0] / population) as u8,
                    (sum[1] / population) as u8,
                    (sum[2] / population) as u8,
                );
                (colour, population)
            })
            .collect()
    }

    fn cut(mut bucket: Vec<Color>) -> (Vec<Color>, Vec<Color>) {
        let mut channel_mins = [u8::MAX, u8::MAX, u8::MAX];
        let mut channel_maxes = [u8::MIN, u8::MIN, u8::MIN];

        for colour in &bucket {
            for i in 0..3 {
                channel_mins[i] = channel_mins[i].min(colour[i]);
                channel_maxes[i] = channel_maxes[i].max(colour[i]);
            }
        }

        let channel_ranges = [
            channel_maxes[0] - channel_mins[0],
            channel_maxes[1] - channel_mins[1],
            channel_maxes[2] - channel_mins[2],
        ];
        let broadest_channel =
            if channel_ranges[0] >= channel_ranges[1] && channel_ranges[0] >= channel_ranges[2] {
                0
            } else if channel_ranges[1] >= channel_ranges[2] {
                1
            } else {
                2
            };

        bucket.sort_by_key(|c| c[broadest_channel]);

        let mid = bucket.len() / 2;
        let second = bucket.split_off(mid);
        (bucket, second)
    }
}

pub mod octree {
    use crate::color::Color;

    #[allow(unused_variables)]
    pub fn quantize(pixels: &[Color], k: usize) -> Vec<(Color, u32)> {
        todo!()
    }
}
