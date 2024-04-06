# Contributing

##  Contribution Workflow

To contribute a patch:

* Fork Repository 
* Create topic branch 
* Commit patches (PR, emails, ...)

In general commits should be atomic and diffs **should be easy to read**.

## Code style guide

Before writing code, please read [our code style](./CODE_STYLE.md).

## Commit format

The commit **must** be formatted in the following way:

```
<context>: <short descriptrion>

<optional description explaining better what happened in the commit>
```

If applicable, link the `issue`/`PR` to be closed with:

* Closes <url>
* Fixes <url>

## Deprecation policy

Where possible, breaking existing APIs should be avoided. Instead, add new APIs and
use [`#[deprecated]`](https://github.com/rust-lang/rfcs/blob/master/text/1270-deprecation.md)
to discourage use of the old one.

Deprecated APIs are typically maintained for one release cycle. In other words, an
API that has been deprecated with the 0.10 release can be expected to be removed in the
0.11 release. This allows for smoother upgrades without incurring too much technical
debt inside this library.

If you deprecated an API as part of a contribution, we encourage you to "own" that API
and send a follow-up to remove it as part of the next release cycle.

## Unwrap and expect

Usage of `.unwrap()` or `.expect("...")` methods is allowed **only** in `examples` or `tests`.

## Coding Conventions

Use `just precommit` or `just check` to format and check the code before committing. This is also enforced by the CI.
