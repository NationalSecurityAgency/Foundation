Cryptol Style Guide
===================

This document was crafted from
[tibbe's Haskell style guide](https://github.com/tibbe/haskell-style-guide/blob/master/haskell-style.md). This
guide covers the major areas of formatting and naming of Cryptol
specifications. That said, the overarching goal of writing a Cryptol
specification is to make it look as much like the corresponding paper
document as possible. So, if there is some clear style in the paper
document, follow that as closely as possible and use this guide to
fill in any gaps.

Formatting
----------

### Line Length

Maximum line length is *70 characters* (which just happens to be the
default `set-fill-column` width in Emacs).

### Indentation

Tabs are illegal. Use spaces for indenting. Indent your code blocks
with *4 spaces*. Indent the `where` keyword two spaces to set it apart
from the rest of the code and indent the definitions in a `where`
clause 2 spaces. For example:

```cryptol
sayHello : {a} (fin a) => [a][8] -> [a+7][8]
sayHello name = greeting
  where
    greeting = "Hello, " # name
```

### Blank Lines

One blank line between top-level definitions. No blank lines between
type signatures and function definitions. No blank lines between a
docstring and the definition it applies to.

### Whitespace

Surround binary operators with a single space on either side. Use your
best judgment for the insertion of spaces around arithmetic
operators, but always be consistent about whitespace on either side of
a binary operator. Don't insert a space after a lambda (the `\`
symbol).

### Type Definitions

It is acceptable to keep type variables, type constraints, and
argument types on one line iff they fit on one line. It is always
acceptable to place type variables, type constraints, and argument
types on separate lines.

```Xcryptol
sayHello :
     {a}
     (fin a)
  => [a][8] -> [a+7][8]
```

The `:` should always be surrounded by a single space on either
side. This aligns type definitions with value definitions (where the
`=` is also surrounded by a single space on either side). For example,

```cryptol
x1 : [32]
x1 = 10
x2 : [4][32]
x2 = [1, 2, 3, 4]
```

### Type Constraints

Type constraints should always be tupleized. The following is
incorrect:

```Xcryptol
func :
    {a, b}
    fin a => fin b =>
    [a][b] -> [a+b]
```

`func` should be:

```Xcryptol
func :
    {a, b}
    (fin a, fin b)
 => [a][b] -> [a+b]
```

### Data Declarations

The base case for `=` should be surrounded by a single space on either
side. This aligns value definitions with type definitions (where the
`:` is also surrounded by a single space on either side). For example,

```cryptol
fish : [32]
fish = 10
```

When multiple definitions are in the same where clause, align the `=`
and/or `:` with the longest definition. For example,

```cryptol
horse    : [4][32]
horse    = [1, 2, 3, 4]
elephant : [2]
elephant = [False, True]
```

Format records as follows:

```cryptol
Person :
    { firstName : String 10
    , lastName  : String 10
    , age       : Integer
    }

Person =
    { firstName = zero # "Han"
    , lastName  = zero # "Solo"
    , age       = 32
    }
```

Align the elements in a list. For example,

```Xcryptol
exceptions =
    [ InvalidStatusCode
    , MissingContentHeader
    , InternalServerError
    ]
```

Optionally, you can skip the first newline. Use your judgment or try
to match the specification you're working from.

```Xcryptol
directions = [ North
             , East
             , South
             , West
             ]
```

### Properties

Short circuit logical operators are preferred to if-then-else in
properties. For example,

```cryptol
property myProperty x =
    x != 0 ==> 100 / x <= 100
```

is superior to

```Xcryptol
property myProperty x =
    if x != 0
    then 100 / x <= 100
    else True
```

### If-then-else clauses

Align if-then-else clauses like you would normal expressions:

```Xcryptol
foo = if   ...
      then ...
      else ...
```

The same rule applies in nested where clauses:

```cryptol
foo a b = c
  where
    c = if a == 0x0000
        then d
          where
            d = if b == 0x0000
                then 0x1000
                else 0x0010
        else 0x0100
```

Comments
--------

### Punctuation

Write proper sentences; start with a capital letter and use proper
punctuation.

### Top-Level Definitions

Comment every top level function (particularly exported functions)
using [docstring comments](https://en.wikipedia.org/wiki/Docstring),
and provide a type signature. The documentation should give enough
information to apply the function without looking at the function's
definition.

When writing a cryptographic algorithm specification in Cryptol, often
the arguments to a function have verbiage in the specification that
may be copied into the function documentation. This is considered a
good practice provided it does not become too verbose. Docstrings
should be written using
[Markdown](https://daringfireball.net/projects/markdown/basics) where
appropriate.

### End-of-Line Comments

Separate end-of-line comments from code using two spaces.

```cryptol
/**
 * Here is a docstring comment for the function named `bar`.
 * This comment will appear in the Cryptol interpreter
 * when you type `:help bar`.
 */
bar : [32] -> [32]
bar n = n % p
  where
    p = 4294967291  // Largest 32-bit prime.
```

Naming
------

When implementing cryptographic algorithm specifications, match
identifiers to the case given in the specification. This helps make
the correspondence clear (especially to those more steeped in
cryptographic algorithm specifications). Should a specification's
identifiers include characters illegal in Cryptol identifiers,
underscore is often a reasonable substitute.

When not constrained by the above it is probably best to follow
Haskell's style:

> Use camel case (e.g. `functionName`) when naming functions and upper
> camel case (e.g. `DataType`) when naming data types.

As well, an apostrophe (`'`) can be appended to a function name to
denote a relationship.

```Xcryptol
encrypt key plainText = f  key plainText

decrypt key plainText = f' key plainText
```

Finally, some tools in the same ecosystem as Cryptol (such as SAW) may
not be able to parse extended characters such as `π`, even though
Cryptol can. Hence, symbols may need to be textualized for toolchain
compatibility.

Curried vs. Uncurried Functions
-------------------------------

[Curried functions](https://en.wikipedia.org/wiki/Currying) are
preferred as they afford [partial
application](https://en.wikipedia.org/wiki/Partial_application) and
tend to reduce the number of parentheses. However, when writing
Cryptol against a specification document, individual functions may
follow the style of the functions from the document (which will
usually be uncurried).

Argument Order
--------------

Order arguments so that partial application is most advantageous. For
example,

```Xcryptol
encrypt key plaintext = ...
```

is superior to

```Xcryptol
encrypt plaintext key = ...
```

since it is easy to see the utility of `encrypt key` as a function in
its own right, e.g. instantiating a mode.

### Warnings ###

All Cryptol specifications must load into the Cryptol interpreter
without warnings (or errors, obviously).
