## Used Tools

This projects uses the following additional tools: 

- [cargo_insta](https://crates.io/crates/cargo-insta): CLI tool to review the changes in the snapshot testing.
- [just](https://github.com/casey/just): as a command runner to automate checks. 
- [typos](https://crates.io/crates/typos): a low positive source code spell checker.

If you want to contribute then make sure to install these tools.

## Reviewing code changes

To validate the quality of the code after some changes, execute the following command
```
just cli
```

This command checks for following things

- Does the rust code compile ?
- Does the rust code adhere to standard linting rules of clippy ?
- Do all unit tests pass ?
- Are there decteable spelling errors ? It focused on low wrong positive errors. 
  Not all spelling errors will be found
- Is the code formatted according to the rust standard

