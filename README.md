# mp3finder

Extracts MPEG Audio frames from arbitrary binary blobs.

## Build

```sh
cargo build --release
```

## Usage

```
mp3finder <in_file> <out_dir>
```

`mp3finder` scans the input file, identifies runs of consecutive valid MPEG Audio frames, and writes each run to a numbered `.mp3` file in the output directory.\
Note that the output directory has to already exist. This tool will not create it for you.

**Example:**

```sh
mp3finder firmware.bin ./extracted/
```
