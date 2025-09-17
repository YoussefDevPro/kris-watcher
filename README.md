# KWIS watcher !!

A command-line tool to watch git repositories and display changes in real-time.

## Installation

To install `kris-watcher`, make sure you have Rust and Cargo installed, then run:

```bash
cargo install kris-watcher
```

## Usage

Once installed, navigate to a git repository and run:

```bash
kris-watcher
```

If you want to make kris auto save youre work, run the cmd using --autosave

```bash
kris-watcher --autosave
```

### Loop Delay

By default, the git watcher loop has a delay of 15 minutes. You can configure this delay using the `-l` or `--loop-delay` flag, followed by a duration. The duration can be in seconds (s, sec), minutes (m, min), or hours (h).

For example, to set the loop delay to 10 seconds:


```bash
kris-watcher -l 10s
```
