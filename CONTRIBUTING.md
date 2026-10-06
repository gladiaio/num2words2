# Contributing to num2words2

`num2words2` is a Rust port of the `savoirfairelinux/num2words` conversion engine with a Python binder, kept output-compatible with the original (see [Relationship to num2words](README.rst#relationship-to-num2words)). Contributions are welcome here directly — open issues and pull requests against [`gladiaio/num2words2`](https://github.com/gladiaio/num2words2), not the upstream repo.

If you have an upstream PR that's been waiting for attention, feel free to open an equivalent PR here and reference the original — we'll port it with full credit to you.

## How can I contribute ?

### Code contribution

#### Issues

If you are unsure where to begin contributing to num2words2, you can start by looking through the issues page.
Numerous issues are created and waiting for your love on the [issue board](https://github.com/gladiaio/num2words2/issues).

#### Pull Requests

Contributions will be accepted through the creation of Pull Requests. Here is the workflow:

* Fork the repository into yours and work from there
* Commit and push your changes into your fork
* When you are done, create a [Pull Request](https://github.com/gladiaio/num2words2/compare) on the **main** branch

A template is provided to create your Pull Request. Try to fill the information at the best of your knowledge.

#### Pull request checklist

For your pull request to be merged, the answer to the following questions must be 'yes':

##### General

* Can the branch be merged automatically?

##### Testing

* Do the unit tests pass?

##### Adding new code

* Is the Python code formatted with black and isort, and does flake8 pass?
* Is the code covered by tests?

GitHub Actions runs these checks (tests on every supported Python version, lint, E2E) on every pull request. See [`num2words2/README.md`](num2words2/README.md) for how to add a language to the Rust core.

### Reporting bugs

Bugs are tracked as [GitHub issues](https://guides.github.com/features/issues/).

#### How to submit a good bug report

Please include as many details as possible. An issue template is automatically loaded when you create an issue.

* Use a clear and comprehensive title for the issue
* Describe the expected behaviour in as many details as possible
* Describe the actual behaviour in as many details as possible

### Testing the application

Our development process is based on Continuous Integration. We love to have a nice code coverage!
