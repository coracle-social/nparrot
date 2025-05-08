<div align="center">
  <img src="media/nparrot.png" alt="Logo" width="200" />
  <h1>nparrot-chat</h1>
</div>

`nparrot-chat` (_"Nostr Parrot Chat"_) is a convenient CLI tool that facilitates messaging over Nostr relay chat rooms.

NB: This project is a fork of [nparrot](https://github.com/Leaf-Computer/nparrot) by Daniel D'Aquino.

This tool has a few use cases and benefits:
- 🤖 Talk to your [Goose AI agent](https://block.github.io/goose/) via chat by using any Nostr client compatible [relays as groups](https://habla.news/u/hodlbod@coracle.social/1741286140797).
- 🔗 Easily integrate almost any other command line tool with Nostr chat messages (be careful about letting other people have access).

Here are the features it includes:
- ✅ **`send` command:** Sends a NIP-C7 message using specified arguments, or from stdin.
- ✅ **`wait` command:** Listens and waits for the next NIP-C7 message from a specific room, and prints it to stdout once received.
- ✅ **`onmessage` command:** Continuously listens for NIP-C7 messages, and for each one, it runs a shell command you specify.
- ✅ **`listen` command:** Continuously listens for NIP-C7 messages, and for each one, it prints it to stdout.
- ✅ **`mcp` command:** MCP server that allows an AI agent to send a message to a specific user, or to wait for their message.

⚠️ **Note:** This is relatively new, experimental software. Please proceed with caution.

# Installation

We currently don't provide pre-built binaries. If you'd like a pre-built binary, please open an issue or contact us, we can help! Otherwise, please follow the build instructions below.

Make sure you have `cargo` 1.85 or higher installed. Then run:

```sh
cargo build --release
```

Then, you can find the executable binary on `./target/release/nparrot-chat`, which you can run from there, or you can move it to another more convenient directory such as `~/.local/bin`.

# Talking to a goose AI agent via Nostr DMs

A very cool use case for this tool is the ability to talk to a [goose AI agent](https://block.github.io/goose/) chat.

NOTE: this isn't recommended unless you fully trust everyone who has access to the room `nparrot-chat` is listening on. Goose is an _agent_, which means it can read and write files on your computer.

1. Copy `.env.template` to `.env`
2. Download any Nostr client that supports NIP-C7 chat messages (e.g. [Flotilla](https://flotilla.social)).
3. Go through onboarding, and join a space. Add the space url to `.env` as `RELAY`.
4. Find the room you'd like your bot to join and get the value of its `h` tag. Add this to `.env` as `ROOM`.
5. Generate a nostr private key (nsec) for your AI agent. If you have [`nak`](https://github.com/fiatjaf/nak), this can be done by simply running:
```
nak key generate | nak encode nsec
```
6. Take note of the `nsec` generated in the previous step and add it to `.env.` as `NSEC`.
7. **Optional but recommended:** Send a message to test if the setup works.
```
nparrot-chat send "test"
```
8. **Optional but recommended:** Modify your `.goosehints` file, to help the AI agent do the right thing.
9. Finally, run this command to start:
```
nparrot-chat onmessage 'goose run --with-extension "nparrot-chat mcp" -i -'
```
10. Try sending a message from your app, and see your AI agent respond to it!

Notes:
- During testing, gpt-4o was used with good results

# Other commands

```
$ nparrot-chat --help
Usage: nparrot-chat --relay <RELAY> --room <ROOM> --nsec <NSEC> <COMMAND>

Commands:
  send       Sends a private message via NIP-17. If the message is omitted, reads it from stdin
  wait       Waits for a private NIP-17 message to be received and prints the decrypted contents to stdout once received
  listen     Listens for private NIP-17 messages to be received and prints the decrypted contents to stdout after each one is received
  mcp        Starts an MCP server to allow an AI agent to manage the conversation
  onmessage  Runs a specified shell command each time it receives a NIP-17 direct message, passing the decrypted message contents to it via stdin
  help       Print this message or the help of the given subcommand(s)

Options:
      --relay <RELAY>  Relay URL to use for sending/receiving messages
      --room <ROOM>    NIP 29 room ID to h-tag when sending/receiving messages
      --nsec <NSEC>    The private key (nsec) identity to sign messages with
  -h, --help           Print help
  -V, --version        Print version
```

## Contributing

Contributions are welcome! Please open an issue or a pull request if you would like to contribute.

Please use `git commit --signoff` when committing changes to this repository, to certify that you agree to the [Developer Certificate of Origin](DCO.txt).
