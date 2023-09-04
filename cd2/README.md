# `cd2` for Smart Directory Navigation

The `cd2` command intelligently predicts the directory you likely want to navigate to based on your input. It does this by utilizing the Jaro-Winkler algorithm to assess the similarity between your input directory and the directories you've previously visited.

## Installation

Supported shell:
- Zsh

Fist, clone the repository:

```sh
git clone https://github.com/Isaac-Fate/rust-cmds.git
```

Change the directory to `cd2`:

```sh
cd cd2
```

Install the command-line with a bash script:

```sh
sh ./install.sh
```

## Usage

For help messages, type:

```sh
cd2 --help
```

Navigate to desired directory via:

```sh
cd2 <Destination>
```

For example,

```sh
cd2 $HOME/Documents
```

sends you to the Documents directory.

Then, later when you are in any other directory, and you intend to navigate to Documents directory, you may do so by simply typing something like:

```sh
cd2 doc
```

This because the text `doc` is very similar to `Documents`.
