# Rust Lambda

## Setup
Install the following to run or deploy this service
- AWS SAM CLI -> [Install Here](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)
- AWS CLI -> [Install Here](https://docs.aws.amazon.com/cli/latest/userguide/install-cliv2.html)
- Make -> [Install Here](http://ftp.gnu.org/gnu/make/)
- Rust -> [Install Here](https://www.rust-lang.org/tools/install)

### Configure AWS CLI
Run the following to configure the AWS CLI

```bash
aws configure
```

AWS Access Key ID [None]: AKIAIOSFODNN7EXAMPLE
AWS Secret Access Key [None]: wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
Default region name [None]: us-west-2
Default output format [None]: json

Set up linux musl
```bash
# adds the x86 64 target to the toolchain
rustup target add x86_64-unknown-linux-musl
# installs the x86 64 toolchain on macOS (for Windows, try cygwin-gcc-linux)
brew install FiloSottile/musl-cross/musl-cross
```

Install musl-cross through brew
```bash
brew install filosottile/musl-cross/musl-cross
```

### Helpful Scripts
Building the project
```bash
cargo build --target x86_64-unknown-linux-musl
```

Deploy the project
```bash
sam build && sam deploy --guided
```

### Helpful Resources
- [Blog Post](https://dev.to/rad_val_/aws-lambda-rust-292g)
- [Example Repo](https://github.com/valentinradu/rust-aws-lambda-example/blob/master/Cargo.toml)
- [Amazon Blog Post](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/)
