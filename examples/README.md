# Examples

This folder contains example scripts that can be used to interact with
the `sample-rust-cdk-app` crate.

## Quickstart

[cargo-rx]: https://github.com/rnag/cargo-rx

Install my crate [cargo-rx], which abstracts away `cargo run --example`.
This provides a single `rx` command.

```shell
❯❯ cargo install cargo-rx
```

Now start out by cloning the BitBucket project:

```shell
❯❯ git clone https://github.com/rnag/sample-rust-cdk-app.git
```

Then, simply just `cd` into the project folder:

```shell
❯❯ cd sample-rust-cdk-app
```

From here, you can use `rx` to build and run
any of the examples individually.

In particular, here's a sample usage of running `examples/upload_ses_templates`:

```shell
❯❯ rx upload_ses_templates
```

If you run the command without any arguments, you can select
from the list of available examples:

```shell
❯❯ rx
```

To pass arguments to a script, you can include them after the `--`.

For example, here's an example of passing arguments to the `sample` script:

```shell
❯❯ rx sample -- \
     -e "prod|sandbox|dev" \
     --dry-run
```

## Setting the AWS Profile

Some scripts might require you to set the AWS profile beforehand (which points to the AWS account, for example `aws-cm-admin-dev`).

Therefore, ensure your $AWS_PROFILE environment is correctly set up:

```shell
❯❯ export AWS_PROFILE='my-profile'
```

## Setting the Log Level

While not necessary, you can also explicitly set the log level for the *example*
as well as the *library* under test, `sample-rust-cdk-app` in this case.

Therefore, remember to ensure that the **RUST_LOG** env variable
is properly set. You can also optionally set the **GLOBAL_RUST_LOG** env
variable, which sets the default log level for external crates and libraries.

For example, on *Mac/Linux*:

```shell
❯❯ export RUST_LOG='TRACE'
❯❯ export GLOBAL_RUST_LOG='WARN'
```

On *Windows*:

```shell
❯❯ $env:RUST_LOG='TRACE'
❯❯ $env:GLOBAL_RUST_LOG='WARN'
```
