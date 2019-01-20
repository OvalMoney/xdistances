.. xdistances documentation master file, created by
   sphinx-quickstart on Thu Jan 24 12:03:17 2019.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to xdistances's documentation!
======================================

.. toctree::
   :hidden:
   :maxdepth: 2
   :caption: Contents:

   self
   reference

Installation
------------

.. code-block:: python

   pip install xdistances

Usage
-----

Go to `<https://xdistances.readthedocs.io>`_ for the full documentation.

Examples
^^^^^^^^

.. code-block:: python

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

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
