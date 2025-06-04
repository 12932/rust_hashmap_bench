# What is this?
A very basic, latency-focused comparison between some easily-available hashers in Rust, with their default settings.

Right now GxHash only builds on nightly

# Results

	const HAYSTACK_SIZE: usize = 128;
	const NEEDLE_STRING_SIZE: usize = 32;
	const HAYSTACK_SEARCH_ITERATIONS: usize = 100_000_000;

On my Macbook M1 Max (2021):
```
Hashing needle string `vwvM5MbpULSkeEK8NMWnlQarjya3OGnT` for 10000000 iterations
[   1739.6958 ms] Std HashMap 
[   1982.6879 ms] SeaHash HashMap 
[    525.5851 ms] FxHashMap 
[    598.5227 ms] AHashMap 
[    457.7710 ms] GxHash 
[  21600.1275 ms] Vector search
```

On my Ryzen 5900x Desktop:

```
Hashing needle string `kX4K5CbCFyHsZsPyb9Il332pBMokgCmL` for 10000000 iterations
[   1930.2627 ms] Std HashMap
[   2811.8873 ms] SeaHash HashMap
[    590.5833 ms] FxHashMap
[    584.7456 ms] AHashMap
[    550.6112 ms] GxHash
[  30315.7427 ms] Vector search
```

Interestingly FxHashMap beats AHash on Apple Silicon
