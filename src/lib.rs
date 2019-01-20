#![deny(clippy::all)]
#![feature(specialization)]

#[macro_use]
extern crate pyo3;
use pyo3::exceptions;
use pyo3::prelude::*;

extern crate strsim;

macro_rules! wrapper {
    ($(#[$doc:meta])* hamming -> $type:ty) => {
        $(#[$doc])*
        #[pyfunction]
        fn hamming(a: &str, b: &str) -> PyResult<$type> {
            match strsim::hamming(a, b) {
                Ok(distance) => Ok(distance),
                Err(_) => Err(exceptions::ValueError::py_err("Lenght mismatch")),
            }
        }
    };
    ($(#[$doc:meta])* $name:ident -> $type:ty) => {
        $(#[$doc])*
        #[pyfunction]
        fn $name(a: &str, b: &str) -> PyResult<$type> {
            Ok(strsim::$name(a, b))
        }
    };
}

wrapper! {
    /// hamming(a, b)
    ///
    /// Calculates the number of positions in the two strings where the characters
    /// differ. Returns an error if the strings have different lengths.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    /// :raises ValueError: if a and b have a different lenghts
    hamming -> usize
}

wrapper! {
    /// levenshtein(a, b)
    ///
    /// Calculates the minimum number of insertions, deletions, and substitutions
    /// required to change one string into the other.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    levenshtein -> usize
}

wrapper! {
    /// osa_distance(a, b)
    ///
    /// Like Levenshtein but allows for adjacent transpositions. Each substring can
    /// only be edited once.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    osa_distance -> usize
}

wrapper! {
    /// damerau_levenshtein(a, b)
    ///
    /// Like optimal string alignment, but substrings can be edited an unlimited
    /// number of times, and the triangle inequality holds.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: int
    damerau_levenshtein -> usize
}

wrapper! {
    /// normalized_levenshtein(a, b)
    ///
    /// Calculates a normalized score of the Levenshtein algorithm between 0.0 and
    /// 1.0 (inclusive), where 1.0 means the strings are the same.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: float
    normalized_levenshtein -> f64
}

wrapper! {
    /// normalized_damerau_levenshtein(a, b)
    ///
    /// Calculates a normalized score of the Damerau–Levenshtein algorithm between
    /// 0.0 and 1.0 (inclusive), where 1.0 means the strings are the same.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: distance
    /// :rtype: float
    normalized_damerau_levenshtein -> f64
}

wrapper! {
    /// jaro(a, b)
    ///
    /// Calculates the Jaro similarity between two strings. The returned value
    /// is between 0.0 and 1.0 (higher value means more similar).
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: similarity
    /// :rtype: float
    jaro -> f64
}

wrapper! {
    /// jaro_winkler(a, b)
    ///
    /// Like Jaro but gives a boost to strings that have a common prefix.
    ///
    /// :param str a: base string
    /// :param str b: string to compare
    /// :return: similarity
    /// :rtype: float
    jaro_winkler -> f64
}

#[pymodule]
fn xdistances(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_function!(hamming))?;
    m.add_wrapped(wrap_function!(levenshtein))?;
    m.add_wrapped(wrap_function!(osa_distance))?;
    m.add_wrapped(wrap_function!(damerau_levenshtein))?;
    m.add_wrapped(wrap_function!(normalized_levenshtein))?;
    m.add_wrapped(wrap_function!(normalized_damerau_levenshtein))?;
    m.add_wrapped(wrap_function!(jaro))?;
    m.add_wrapped(wrap_function!(jaro_winkler))?;
    Ok(())
}
