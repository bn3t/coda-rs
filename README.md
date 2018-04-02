# coda-rs - Parse Coda files [![Build Status](https://travis-ci.org/bn3t/coda-rs.svg?branch=master)](https://travis-ci.org/bn3t/coda-rs)

Utility to parse coda files (https://www.febelfin.be/sites/default/files/files/standard-coda-2.5-en.pdf).

This is a utility to tail text from an HTTP endpoint. It works similarly to doing tail -f with a local file. Typically, the source should be a growing text file (like a log file) served by a server supporting the Range header in http.

## Usage

```
$ coda-rs -h                                                                                                            Usage:
    coda-rs [OPTIONS] CODA

Parse coda files

positional arguments:
  coda                  Coda file to parse

optional arguments:
  -h,--help             show this help message and exit
  -j,--json             Convert coda files to json
  -v,--version          Show version
```

### Example

```
coda-rs --json FILE.CD2
```

## Features

* Parse Header
* Parse old balance (1)
* Parse movement record (2.1)
* Parse movement record (2.2)
* Parse movement record (2.3)

### TODO

* Generate JSON file
* Parse information record (3.1)
* Parse information record (3.2)
* Parse information record (3.3)
* Parse new balance (8)
* Parse free communication (4)
* Parse trailer record (9)
