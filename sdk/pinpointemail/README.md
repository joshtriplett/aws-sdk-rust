# aws-sdk-pinpointemail

**Please Note: The SDK is currently released as an alpha and is intended strictly for
feedback purposes only. Do not use this SDK for production workloads.**

Welcome to the _Amazon Pinpoint Email API Reference_. This guide provides information about the Amazon Pinpoint Email API (version 1.0), including supported operations, data types, parameters, and schemas.

[Amazon Pinpoint](https://aws.amazon.com/pinpoint) is an AWS service that you can use to engage with your customers across multiple messaging channels. You can use Amazon Pinpoint to send email, SMS text messages, voice messages, and push notifications. The Amazon Pinpoint Email API provides programmatic access to options that are unique to the email channel and supplement the options provided by the Amazon Pinpoint API.

If you're new to Amazon Pinpoint, you might find it helpful to also review the [Amazon Pinpoint Developer Guide](https://docs.aws.amazon.com/pinpoint/latest/developerguide/welcome.html). The _Amazon Pinpoint Developer Guide_ provides tutorials, code samples, and procedures that demonstrate how to use Amazon Pinpoint features programmatically and how to integrate Amazon Pinpoint functionality into mobile apps and other types of applications. The guide also provides information about key topics such as Amazon Pinpoint integration with other AWS services and the limits that apply to using the service.

The Amazon Pinpoint Email API is available in several AWS Regions and it provides an endpoint for each of these Regions. For a list of all the Regions and endpoints where the API is currently available, see [AWS Service Endpoints](https://docs.aws.amazon.com/general/latest/gr/rande.html#pinpoint_region) in the _Amazon Web Services General Reference_. To learn more about AWS Regions, see [Managing AWS Regions](https://docs.aws.amazon.com/general/latest/gr/rande-manage.html) in the _Amazon Web Services General Reference_.

In each Region, AWS maintains multiple Availability Zones. These Availability Zones are physically isolated from each other, but are united by private, low-latency, high-throughput, and highly redundant network connections. These Availability Zones enable us to provide very high levels of availability and redundancy, while also minimizing latency. To learn more about the number of Availability Zones that are available in each Region, see [AWS Global Infrastructure](http://aws.amazon.com/about-aws/global-infrastructure/).

## Getting Started

> Examples are availble for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/sdk/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-pinpointemail` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = "0.0.22-alpha"
aws-sdk-pinpointemail = "0.0.22-alpha"
tokio = { version = "1", features = ["full"] }
```

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Guide](https://github.com/awslabs/aws-sdk-rust/blob/main/Guide.md). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) – For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/sdk/examples)

## License

This project is licensed under the Apache-2.0 License.
