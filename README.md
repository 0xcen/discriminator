# Anchor Discriminator Generator

`anchor_discriminator_generator` is a CLI tool written in Rust that generates the anchor discriminator for your instruction or struct's name from a combination of a namespace and a name.

The most common namespace `"global"` is set as default.

## Usage

To use `discriminator`, you'll need to run it from the command line. Here's how to use it:

```
discriminator [OPTIONS] NAME
```

The program takes one required argument, `NAME`, which is the name to be combined with the namespace to generate the hash.

### Options

`-n, --namespace`: Sets the namespace to use when generating the hash. If this option is not provided, the default namespace of `"global"` will be used.

### Example Usage

Here are some examples of how to use `anchor_discriminator_generator`:

Generate a hash using the default namespace of "global"
```
$ discriminator my_name
namespace: global
name: my_name
hash: [195, 62, 35, 70, 109, 102, 115, 85]
```

Generate a hash using a custom namespace
```
$ discriminator -n my_namespace my_name
namespace: my_namespace
name: my_name
hash: [71, 239, 96, 91, 126, 146, 191, 3]
```
## Building

To build `anchor_discriminator_generator` from source, you'll need to have Rust installed on your machine. Once you've installed Rust, you can build the program by running the following command from the root of the project directory:

```
$ cargo build --release
```

This will compile the program and create an executable file in the `target/release` directory. To run the program, navigate to the `target/release` directory and run the executable with `./discriminator` (replace `discriminator` with the name of your executable).

## License

This program is licensed under the [MIT License](LICENSE).
