# What is this?
A very basic comparison between some easily-available hashers in Rust, with their default settings.

# Results

	const HAYSTACK_SIZE: usize = 128;
	const NEEDLE_STRING_SIZE: usize = 32;
	const HAYSTACK_SEARCH_ITERATIONS: usize = 10_000_000;

On my Macbook M1 Max (2021):
```
Hashing needle string `lHI3kla0eOUe9WC2bR6wLJMziOy81iju` for 10000000 iterations
[    224.8060 ms] Std HashMap
[    150.6508 ms] SeaHash HashMap
[     46.1382 ms] FxHashMap
[     61.2352 ms] AHashMap
[   2119.9355 ms] Vector search
```

On my Ryzen 5900x Desktop:

```
Hashing needle string `ltt9unmTMMbGBD2H1sjgHCL5h4YjCGHO` for 10000000 iterations
[    180.8723 ms] Std HashMap
[    262.1531 ms] SeaHash HashMap
[     80.1281 ms] FxHashMap
[     64.1460 ms] AHashMap
[   2582.2459 ms] Vector search
```

Interestingly FxHashMap beats AHash on Apple Silicon