# Sample Rust CDK App

> A [Cargo] project written in [Rust], which uses the [CDK v2] for Typescript.

[Cargo]: https://doc.rust-lang.org/cargo/
[Rust]: https://www.rust-lang.org/
[CDK v2]: https://aws.amazon.com/about-aws/whats-new/2021/12/aws-cloud-development-kit-cdk-generally-available/

You should explore the contents of this project. It demonstrates a CDK app with an instance of a stack (`SampleRustCDKAppStack`)
which contains an Amazon SQS queue that is subscribed to an Amazon SNS topic.

The `cdk.json` file tells the CDK Toolkit how to execute your app.

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `cdk deploy`      deploy this stack to your default AWS account/region
* `cdk diff`        compare deployed stack with current state
* `cdk synth`       emits the synthesized CloudFormation template
* `cdk docs`        open CDK documentation

Enjoy!

## Deployment

First things first:
1. Ensure you have the [Rust] compiler and `cargo` installed. This project has been tested with `rustc` version **1.62.1**.
2. Run `npm i` in the project directory to install all *Node.js* dependencies.
3. Follow instructions under *Getting Started* in the [rust.aws-cdk-lambda](https://www.npmjs.com/package/rust.aws-cdk-lambda) project.

Now that's out of the way, here's how you'd do a stack deployment or update to a target AWS account:
1. Set the `AWS_PROFILE` environment variable to point to the AWS account to deploy
   the CDK app to.
2. Synthesize and deploy the stack with the `cdk` tool:
   ```shell
   cdk deploy
   ```

> *Tip:* I find it helpful to create alias commands and add them to `~/.bash_aliases` or `~/.bashrc`, for example like `alias c="cdk deploy"`.

## Running Local Scripts

The local *example* scripts can be run, for testing or debugging purposes as needed.

For more info, check out the [README.md](./examples/README.md) under the `examples/` directory.
