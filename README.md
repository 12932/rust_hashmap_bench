# What is this?
A very basic comparison between some easily-available hashers in Rust, with their default settings.

Right now GxHash only builds on nightly

# Results

	const HAYSTACK_SIZE: usize = 128;
	const NEEDLE_STRING_SIZE: usize = 32;
	const HAYSTACK_SEARCH_ITERATIONS: usize = 10_000_000;

On my Macbook M1 Max (2021):
```
Hashing needle string `vwvM5MbpULSkeEK8NMWnlQarjya3OGnT` for 10000000 iterations
[    178.1681 ms] Std HashMap
[    157.0172 ms] SeaHash HashMap
[     59.9349 ms] FxHashMap
[     56.3037 ms] AHashMap
[     58.5313 ms] GxHash
[   2140.3751 ms] Vector search
```

On my Ryzen 5900x Desktop:

```
Hashing needle string `kX4K5CbCFyHsZsPyb9Il332pBMokgCmL` for 10000000 iterations
[    196.7708 ms] Std HashMap
[    347.4592 ms] SeaHash HashMap
[     85.6588 ms] FxHashMap
[     82.5427 ms] AHashMap
[     79.4092 ms] GxHash
[   3027.2902 ms] Vector search
```

Interestingly FxHashMap beats AHash on Apple Silicon