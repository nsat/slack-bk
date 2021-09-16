# Changelog

Notable changes to slack-bk are tracked in this file.

The format is based on the [Keep A Changelog 1.0.0](https://keepachangelog.com/en/1.0.0/) spec.
Releases may be found on [GitHub](https://github.com/nsat/slack-bk/releases/) and are tagged with
their release number in the Git repository. Release numbers follow the [Semantic Versioning
2.0.0](https://semver.org/) format. As a reminder, this format uses major, minor, and patch numbers
with the following form:

```
v1.2.3-test
 ^ ^ ^ ^
 | | | |
 | | | pre-release tag
 | | patch
 | minor
 major
```

These are incremented according to the following rules:

- *MAJOR* versions contain *backwards-incompatible changes*.
- *MINOR* versions contain new *backwards-compatible* features.
- *PATCH* versions contain *backwards-compatible* fixes.

## Types of changes

_Added_ for new features.
_Changed_ for changes in existing functionality.
_Deprecated_ for soon-to-be removed features.
_Removed_ for now removed features.
_Fixed_ for any bug fixes.
_Security_ in case of vulnerabilities.

### A note to release managers

When creating a new release in GitHub, please copy the `[Unreleased]` section to a new versioned
section and use it for the release's notes, in addition to verifying that version numbers are
updated throughout the repository.

## [Unreleased]

### Added
- Most types now derive `Default` to improve the ergonomics of constructing messages.

## v0.1.0

### Added
- Initial release of `slack-bk`
