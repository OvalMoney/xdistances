# xdistances

Python wrapper on [strsim](https://crates.io/crates/strsim) a [Rust](https://www.rust-lang.org) implementations of [string similarity metrics]:
  - [Hamming]
  - [Levenshtein] - distance & normalized
  - [Optimal string alignment]
  - [Damerau-Levenshtein] - distance & normalized
  - [Jaro and Jaro-Winkler] - this implementation of Jaro-Winkler does not limit the common prefix length

The normalized versions return values between `0.0` and `1.0`, where `1.0` means
an exact match.

## Installation

`pip install xdistances`

## Usage

Go to [https://xdistances.readthedocs.io](https://xdistances.readthedocs.io/en/latest/) for the full documentation.

### Examples

```python
>>> import xdistances
>>> xdistances.hamming("hamming", "hammers")
3
>>> xdistances.hamming("hamming", "hammer")
Traceback (most recent call last):
  File "<stdin>", line 1, in <module>
ValueError: Lenght mismatch
>>> xdistances.levenshtein("kitten", "sitting")
3
>>> xdistances.normalized_levenshtein("kitten", "sitting")
0.5714285714285714
>>> xdistances.osa_distance("ac", "cba")
3
>>> xdistances.damerau_levenshtein("ac", "cba")
2
>>> xdistances.normalized_damerau_levenshtein("levenshtein", "löwenbräu")
0.2727272727272727
>>> xdistances.jaro("Friedrich Nietzsche", "Jean-Paul Sartre")
0.39188596491228067
>>> xdistances.jaro_winkler("cheeseburger", "cheese fries")
0.9111111111111111
```

## Contributing

If you don't want to install Rust itself, you can run `$ ./dev` for a
development CLI if you have [Docker] installed.

Benchmarks require a Nightly toolchain. Run `$ cargo +nightly bench`.

## Credits

strsim: [crates](https://crates.io/crates/strsim) - [Github](https://github.com/dguo/strsim-rs)

## License

[MIT](https://github.com/OvalMoney/xdistances/blob/master/LICENSE)

[string similarity metrics]:http://en.wikipedia.org/wiki/String_metric
[Damerau-Levenshtein]:http://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance
[Jaro and Jaro-Winkler]:http://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance
[Levenshtein]:http://en.wikipedia.org/wiki/Levenshtein_distance
[Hamming]:http://en.wikipedia.org/wiki/Hamming_distance
[Optimal string alignment]:https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance#Optimal_string_alignment_distance
[Docker]:https://docs.docker.com/engine/installation/
