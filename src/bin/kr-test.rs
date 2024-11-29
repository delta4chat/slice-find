use slice_find::*;

use core::time::Duration;

use std::time::Instant;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use std::thread;

fn rand_bytes(size: usize) -> Vec<u8> {
    let mut out = Vec::new();
    while out.len() < size {
        out.extend(RandomState::new().build_hasher().finish().to_ne_bytes());
    }
    while out.len() > size {
        out.pop();
    }
    out
}

fn stress_one(algo: Algorithm) -> Vec<Duration> {
    let rand_len: usize = 100 +
        ((u16::from_ne_bytes({
            let mut x = [0, 0];
            x.copy_from_slice(&rand_bytes(2));
            x
        }) % 10240) as usize);
    let haystack = rand_bytes(rand_len);
    let needles: Vec<&[u8]> = haystack.windows(haystack.len()/40).collect();

    let mut times = Vec::new();
    //dbg!(format!("{:?}", &haystack));
    for needle in needles.iter() {
        //dbg!(format!("{:?}", needle));
        let t = Instant::now();
        let maybe_pos = algo.slice_find(&haystack, needle);
        let t = t.elapsed();
        times.push(t);
        //dbg!(format!("{maybe_pos:?}"));
        assert!( maybe_pos.is_some() );
        let pos = maybe_pos.unwrap();
        assert_eq!(&haystack[pos .. pos+needle.len()], *needle);
        let mut needle_rev: Vec<u8> = needle.iter().rev().map(|x| { *x }).collect();
        needle_rev = needle_rev.repeat(3);
        assert!( algo.slice_find(&haystack, &needle_rev).is_none() );
    }

    times

}

fn stress(name: &str, algo: Algorithm) {
    let mut times = Vec::new();
    for _ in 0..100 {
        times.extend(&stress_one(algo));
    }

    let sum: f64 = times.iter().map(|x: &Duration| {x.as_secs_f64() }).sum();
    let len = times.len() as f64;

    let avg =
        if len == 0.0 {
            String::from("N/A")
        } else {
            format!("{:?}", Duration::from_secs_f64(sum / len))
        };
    println!("{name:09}: average used time in single operation = {} | total used time = {:?}", avg, Duration::from_secs_f64(sum));
}

fn main() {
    let mut thrs = Vec::new();
    thrs.push(thread::spawn(|| { stress("KMP", Algorithm::KMP) }));
    thrs.push(thread::spawn(|| { stress("Raita", Algorithm::Raita) }));
    thrs.push(thread::spawn(|| { stress("Simple", Algorithm::Simple) }));
    for thr in thrs.into_iter() {
        let _r = thr.join();
        //eprintln!("{_r:?}");
    }
}
