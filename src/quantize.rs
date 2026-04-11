pub mod median_cut {
    use crate::colour::Colour;

    pub fn quantize(pixels: &[Colour], k: usize) -> Vec<Colour> {
        let mut buckets: Vec<Vec<Colour>> = vec![pixels.to_vec()];
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

        let mut result: Vec<Colour> = vec![];
        for bucket in buckets {
            let result_colour = bucket.iter().fold([0u32; 3], |mut acc, c| {
                acc[0] += c[0] as u32;
                acc[1] += c[1] as u32;
                acc[2] += c[2] as u32;
                acc
            });
            let len = bucket.len() as u32;
            result.push(Colour::new(
                (result_colour[0] / len) as u8,
                (result_colour[1] / len) as u8,
                (result_colour[2] / len) as u8,
            ));
        }

        result
    }

    fn cut(mut bucket: Vec<Colour>) -> (Vec<Colour>, Vec<Colour>) {
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
    use crate::colour::Colour;

    pub fn quantize(pixels: &[Colour], k: usize) -> Vec<Colour> {
        todo!()
    }
}
