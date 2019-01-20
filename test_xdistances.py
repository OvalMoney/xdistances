import pytest
import xdistances


def test_hamming():
    assert xdistances.hamming("a", "a") == 0
    assert xdistances.hamming("a", "b") == 1
    with pytest.raises(ValueError):
        xdistances.hamming("a", "aa")
    assert xdistances.hamming("aaaa", "aabb") == 2


def test_osa_distance():
    assert xdistances.osa_distance("a", "a") == 0
    assert xdistances.osa_distance("a", "b") == 1
    assert xdistances.osa_distance("aaaa", "aabb") == 2


def test_levenshtein():
    assert xdistances.levenshtein("a", "a") == 0
    assert xdistances.levenshtein("a", "b") == 1
    assert xdistances.levenshtein("aaaa", "aabb") == 2


def test_normalized_levenshtein():
    assert xdistances.normalized_levenshtein("a", "a") == 1.0
    assert xdistances.normalized_levenshtein("a", "b") == 0.0
    assert xdistances.normalized_levenshtein("aaaa", "aabb") == 0.50


def test_damerau_levenshtein():
    assert xdistances.damerau_levenshtein("a", "a") == 0
    assert xdistances.damerau_levenshtein("a", "1") == 1
    assert xdistances.damerau_levenshtein("aaaa", "aabb") == 2


def test_normalized_damerau_levenshtein():
    assert xdistances.normalized_damerau_levenshtein("a", "a") == 1.0
    assert xdistances.normalized_damerau_levenshtein("a", "b") == 0.0
    assert xdistances.normalized_damerau_levenshtein("aaaa", "aabb") == 0.50


def test_jaro():
    assert xdistances.jaro("a", "a") == 1.0
    assert xdistances.jaro("a", "b") == 0.0
    assert xdistances.jaro("aaaa", "abbb") == 0.5


def test_jaro_winkler():
    assert xdistances.jaro_winkler("a", "a") == 1.0
    assert xdistances.jaro_winkler("a", "b") == 0.0
    assert xdistances.jaro_winkler("aaaa", "abbb") == 0.55

