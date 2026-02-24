# NewC

NewC is a 'dialect' of C which adds a few features:

- `defer` keyword: similar to Zig's `defer` keyword; makes a statement run at the end of a scoped block.
    - If scope is exited early with `return`, the statement is NOT ran. This will hopefully be added soon.
- UFCS, a.k.a. pseudo-methods: a function can be called as `foo.bar(baz)`, and it will be translated to `bar(foo, baz)`.

If you would like to use NewC, there is a template availible at [akiramusic000/ncc-template](https://github.com/akiramusic000/ncc-template).
