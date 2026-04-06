# mp3finder

Extracts MPEG Audio frames from arbitrary binary blobs.

## Build

```sh
cargo build --release
```

## Usage

```
mp3finder [OPTIONS] <in_file> <out_dir>
```

`mp3finder` scans the input file, identifies runs of consecutive valid MPEG Audio frames, and writes each run to a numbered `.mp3` file in the output directory.\
Note that the output directory has to already exist. This tool will not create it for you.

### Options:

* `-f`/`--frame-limit <N>` Skip runs with fewer than N frames

### Example:

```sh
mp3finder firmware.bin ./extracted/

# to only export mp3 files consisting of 100 frames or more
mp3finder -f 100 firmware.bin ./extracted/
```
