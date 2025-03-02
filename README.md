# token-source
Providing a high level API for authentication token source providers.

![CI](https://github.com/nicolas-vivot/token-source/workflows/CI/badge.svg?branch=main)
[![crates.io](https://img.shields.io/crates/v/token-source.svg)](https://crates.io/crates/token-source)


## Keep it simple

- Only trait definitions.
- Keep it tech/vendor agnostic.
- As light as possible (limited dependencies).


## Motivations behind this library

### Main problematic

Libraries that deal with authentication can be divided into two parts:
- the providers: the ones that perform the authentication and provide tokens. Example: [Google Cloud Rust][link-google-cloud-rust].
- the consumers: the ones that require you to provide tokens for their usage. Example: [Reqwest][link-reqwest], [Openapi Generator][link-openapi-generator].

Unfortunately, nothing really unify these two categories of library.
Also, because some libraries may provide you very limited way to manage the tokens, it often leave you the responsibility to bridge the gap between the two.

**Example 1**

You are deploying a service in Kubernetes (let's say GKE) and this servive need to request another one.
- the external service provides a rust client library, automatically generated via [Openapi Generator][link-openapi-generator]. This requires to set the authentication as a `string` when creating the client.
- use [Google Cloud Rust][link-google-cloud-rust] or any other crate providing similar features to get a valid authentication token.

It will come a time where a generated token will expire (especially short-time ones) and you probably do not want to create a new client instance for every request your need to perform to the external service.
Here, because of the lack of functionalities/authentication mechanism support from the generated library, you have no other choice than dealing with the token expiry verification and renewal by yourself, re-creating a new client.

**Example 2**

You are deploying a service in Kubernetes (let's say GKE) and this service need to request another one.
You may use:
- use [Reqwest][link-reqwest] to query the external service, providing a token as a `string` put in the headers.
- use [Reqwest Retry][link-reqwest-middleware] because you want to add some retry policy/strategy.
- use `google-cloud-rust` or any other crate providing similar features to get a valid authentication token.

Everything may goes well, until one of your request get triggered very close to the expiry time of your already generated token and that request fails and get retries for some reason. When the retry policy applys, the authentication token will expire and the subsequent retries will all fails due to `403`

**Example 3**

Same as example 2, but let says that you now want to deploy your service in both GKE and EKS.
You will have to provide two different way of sourcing the authentication tokens. You can use any technic (based on environment settings, based on compilation profiles) but your will certainly have to write some code to deal with this, and that can be pretty boiler plate depending on the libraries you will use.

### One crate to standardize them all

Having a single crate with simple definitions of `token-source` and `token-source-provider` as Trait will help to standardize expectations, behavior and responsibilities across libraries.

As other libraries start to use the `TokenSource` and `TokenSourceProvider` traits in place of raw of custom types, it will become easier to use it.

### What benefits does this bring?
- vendor agnostic.
- better abstraction, separation of concerns and responsibilities.
- enhanced cross library integration.
- less DIY glue code.

## License
This project is licensed under the [MIT license](./LICENCE).

## Contributing
Contributions are welcome.
1. Fork this repository.
2. Make changes, commit to your fork.
3. Send a pull request with your changes.
4. Confirm the success of CI.


[link-google-cloud-rust]: https://github.com/yoshidan/google-cloud-rust
[link-openapi-generator]: https://github.com/OpenAPITools/openapi-generator
[link-reqwest]: https://github.com/seanmonstar/reqwest
[link-reqwest-middleware]: https://github.com/TrueLayer/reqwest-middleware