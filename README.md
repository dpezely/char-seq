char-seq
========

Mapping **characters** of various natural language scripts to a compact,
contiguous **sequence**.

This solves the problem of Unicode code-points for a language such as
Croatian requiring characters from the "common" Latin range plus an
"extended" range as legacy of ASCII and ISO-8859.

One use case is
[anagram-phrases](https://github.com/dpezely/anagram-phrases/) solver,
which translates each character to a unique prime number.

This library *initially* exists only to serve that software and is unlikely
to be suitable for more general use cases.

Pull requests to change that, of course, are welcome.

## Caveat

Please be mindful that this may only be used for a single natural language
at a time.

For instance, pick *one*: Albanian, English, French, Greek, Italian, etc.

For using multiple languages, encapsulate and then instantiate once per
language.

This tool is ignorant of language semantics and deals only with scripts.

Understand the difference between "language" versus "script" for purposes
here and as defined by the Unicode standard.  Each of English and French is
a natural *language*, yet both use the Latin alphabet.  When applying
accents upon one language's use of otherwise similar characters such as
`'c'` in "Français" or `'e'` in "vérité", then that character set is called
a *script*.  Here, we use "script" and "character set" almost
interchangeably.

## Example

A value read as Latin-1 that actually should be Latin-2 and gets converted
using strict methods to UTF-8 would result in an incorrect character.

In the Croatian language, the character `š` representing the "sh" sound has
Unicode value U+0161.  In Latin-2 (ISO-8859-2), it is 0xB9.  That same code
in Latin-1 (ISO8859-1) is `¹` indicating subscript of numeral one.

By this example, it should be obvious why we cannot and shoud not naïvely
remap values in hopes that an appropriate font rendering would somehow solve
the problem.

## Performance Impact

For a given natural language, there may be several Unicode code-point
ranges, each of which seemingly encoding upercase, lowercase and caseless
characters differently.

Because of this, the code here uses a rather lengthy set of `if`/`else if`
conditionals.

Until proper load & capacity measurement and testing can be performed...

Performance impact using the anagram-phrases package appears negligible for
English language word phrase as input and loading the `wamerican-huge`
dictionary, available as an optional install via:

    sudo apt-get install wamerican-huge
