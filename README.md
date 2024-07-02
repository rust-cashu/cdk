# Cashu Development Kit

> [!WARNING]  
> Archived! Check <https://github.com/cashubtc/cdk>.

## Project structure

The project is split up into several crates in the `crates/` directory:

* Libraries:
    * [**cdk**](crates/cdk): Cashu Development Kit
* Binaries (tools):
    * [**cashu-cli**](./crates/cashu-cli): Cashu CLI

### Bindings

**cdk** crate can be embedded inside other environments, like Swift, Kotlin, Python and JavaScript.
Please, explore the [`bindings/`](./bindings) directory to learn more.

## State

**These libraries are in ALPHA state**, things that are implemented generally work but the API will change in breaking ways.

## License

This project is distributed under the MIT software license - see the [LICENSE](LICENSE) file for details
