# Contributing to rust-lpc82x

Thank you for considering to work on rust-lpc82x. This document will give you some pointers and explain the guidelines that you need to follow.

## Opening issues

If you found a problem, please open an issue on the [GitHub repository]. If you're not sure whether you found a real problem or not, just open an issue anyway. We'd rather close a few invalid issues than miss anything relevant.

## Contributing changes

If you want to contribute a change to rust-lpc82x, please open a pull request on the [GitHub repository]. The best way to open a pull request is usually to just push a branch to your fork, then click the button that appears near the top of your fork's GitHub page.

If you're having any problems with completing your change, feel free to open a pull request anyway and ask any questions there. We're happy to help you get your changes across the finish line.

Please note that all Rust code is automatically generated from an [SVD file] using [svd2rust]. Manual changes to Rust code will usually not be accepted. If there's a problem with the generated Rust code, that means either the SVD file or svd2rust are doing something wrong.

### Patching the SVD file

There's existing infrastructure for patching the SVD file on the fly in the [update script]. To fix an issue in the SVD file, create another patch (see the patches at the root of the repository for examples) and add it to the update script.


## Release Procedure

This section is intended for project maintainers only. It assumes that you can push to the repository (here called `upstream`, but primarily work on your own fork (`origin`),

1. Check out feature branch for the release (replace x.y.z with actual version)
```
$ git checkout -b release-x.y.z
```

2. Push feature branch to your fork (required for the next steps to work)
```
$ git push -u origin release-x.y.z
```

3. Update changelog

4. Update versions in README.md, if version bump is major or minor

5. Do cargo-release dry run, review its actions
```
$ cargo release --level major|minor|patch --dry-run
```

6. Run cargo-release
```
$ cargo release --level major|minor|patch
```

7. Open pull request, review it, merge it

8. Push the tag that cargo-release created to `upstream`
```
git checkout master
git pull upstream master
git push --tag upstream master
```


That's it! If anything about this document is unclear, feel free to open an issue. If you have questions regarding a pull request that you're working on, just open the pull request and ask your questions there.

[GitHub repository]: https://github.com/braun-robotics/rust-lpc82x
[SVD file]: http://ds.arm.com/media/resources/db/chip/nxp/lpc824m201jdh20/LPC82x.svd
[svd2rust]: https://crates.io/crates/svd2rust
[update script]: https://github.com/braun-robotics/rust-lpc82x/blob/master/scripts/update
