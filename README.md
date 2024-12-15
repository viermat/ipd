# ipd

`ipd` is a lightweight and user-friendly command-line tool designed to convert IP addresses into non-standard numeric formats, including:

- Hexadecimal: 0x7f.0x0.0x0.0x1
- Octal: 0177.00.00.01
- Decimal: 2130706433

### Use cases

Representing IP addresses in different numeric formats can be useful in a range of scenarios, including security research. For example, an attacker might use an obfuscated IP address to exploit system vulnerabilities, triggering errors that could reveal sensitive information about the system.

# Getting started

### Building from source

To build `ipd` from source, you need Rust 1.80.0 or later. Clone the repository and use `cargo` to build and install it:

```bash
cargo install --path .
```

To verify a successful installation, simply run:

```bash
ipd -h
```

# How to use

## Basic usage

`ipd` will have `output` set as `decimal` by default:

```bash
$ ipd 127.0.0.1
127.0.0.1 2130706433
```

`ipd` can also accept multiple IP addresses as arguments:

```bash
$ ipd 127.0.0.1 192.168.100.1 0.255.0.255
127.0.0.1 2130706433
192.168.100.1 3232261121
0.255.0.255 16711935
```

## Advanced usage

To hide the IP address in the output, pass the `--quiet` flag:

```bash
$ ipd -q 127.0.0.1
2130706433
```

To change the output format, pass the `--output` flag followed by `hex`, `dec`, `oct` or `all`:

```bash
$ ipd -q -o hex 127.0.0.1 
0x7f000001
```

You can display the full versions of the formats by using the `--full` flag:

```bash
$ ipd -o all -f 127.0.0.1
127.0.0.1 2130706433 0x7f.0x0.0x0.0x1 0177.00.00.01
```

Alternatively, you can pipe input directly to `ipd` by passing `-` as a positional argument:

```bash
$ echo "127.0.0.1\n192.168.100.1" | ipd -o hex -
127.0.0.1 0x7f000001
192.168.100.1 0xc0a86401
```

> [!WARNING]  
> `ipd` expects input IPs to be separated by newlines only and will ignore ALL whitespace characters