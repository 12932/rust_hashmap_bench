use gxhash::GxHashMap;
use seahash::SeaHasher;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;
use std::iter::repeat_with;

use ahash::{AHashMap, HashMapExt};
use rustc_hash::FxHashMap;

type SeaHashBuilder = BuildHasherDefault<SeaHasher>;

/// Log how long it took to execute `$tt`, using `$reason` as the status
/// message of what was timed
macro_rules! measure {
    ( $reason:expr, $tt:block ) => {{
        let it = std::time::Instant::now();
        let ret = $tt;
        let elapsed = it.elapsed().as_secs_f64() * 1000.;
        println!("[{elapsed:12.4} ms] {}", $reason);
        ret
    }};
}

// generate random ascii string of n size
fn generate_random_string(length: usize) -> String {
    repeat_with(fastrand::alphanumeric).take(length).collect()
}

fn main() {
    const HAYSTACK_SIZE: usize = 128;
    const NEEDLE_STRING_SIZE: usize = 32;
    const HAYSTACK_SEARCH_ITERATIONS: usize = 10_000_000;

    let my_needle_string: String = generate_random_string(NEEDLE_STRING_SIZE);

    println!(
        "Hashing needle string `{}` for {} iterations",
        my_needle_string, HAYSTACK_SEARCH_ITERATIONS
    );
    let mut my_haystack_vector: Vec<String> = Vec::new();

    // Push random strings into the vector
    for _ in 0..HAYSTACK_SIZE {
        my_haystack_vector.push(generate_random_string(NEEDLE_STRING_SIZE));
    }
    // Put the needle in the middle
    my_haystack_vector.push(my_needle_string.clone());
    // Add more strings onto the end
    for _ in 0..HAYSTACK_SIZE {
        my_haystack_vector.push(generate_random_string(NEEDLE_STRING_SIZE));
    }

    let mut my_hash_map = HashMap::new();

    let hash_string = my_needle_string.clone();
    my_hash_map.insert(hash_string, true);

    // Std HashMap with default hasher
    let mut std_hash_map = HashMap::new();
    std_hash_map.insert(my_needle_string.clone(), true);
    measure!("Std HashMap", {
        for _ in 0..HAYSTACK_SEARCH_ITERATIONS {
            let _needle_found = std_hash_map.contains_key(&my_needle_string);
        }
    });

    // SeaHashMap
    let mut seahash_hash_map: HashMap<String, bool, SeaHashBuilder> =
        HashMap::with_hasher(SeaHashBuilder::default());

    seahash_hash_map.insert(my_needle_string.clone(), true);
    measure!("SeaHash HashMap", {
        for _ in 0..HAYSTACK_SEARCH_ITERATIONS {
            let _needle_found = seahash_hash_map.contains_key(&my_needle_string);
        }
    });

    // FxHashMap
    let mut fx_hash_map: FxHashMap<String, bool> = FxHashMap::default();
    fx_hash_map.insert(my_needle_string.clone(), true);
    measure!("FxHashMap", {
        for _ in 0..HAYSTACK_SEARCH_ITERATIONS {
            let _needle_found = fx_hash_map.contains_key(&my_needle_string);
        }
    });

    // AHashMap
    let mut a_hash_map: AHashMap<String, bool> = AHashMap::new();
    a_hash_map.insert(my_needle_string.clone(), true);
    measure!("AHashMap", {
        for _ in 0..HAYSTACK_SEARCH_ITERATIONS {
            let _needle_found = a_hash_map.contains_key(&my_needle_string);
        }
    });

    // GxHash
    let mut gx_hash_map: GxHashMap<String, bool> = GxHashMap::new();
    gx_hash_map.insert(my_needle_string.clone(), true);
    measure!("GxHash", {
        for _ in 0..HAYSTACK_SEARCH_ITERATIONS {
            let _needle_found = gx_hash_map.contains_key(&my_needle_string);
        }
    });

    // Vector search check
    measure!("Vector search", {
        for _ in 0..HAYSTACK_SEARCH_ITERATIONS {
            let _needle_found = my_haystack_vector.contains(&my_needle_string);
            //print!("{:?}", needle_found) ;
        }
    });
}
