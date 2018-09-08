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
  -e,--encoding ENCODING
                        Encoding for reading, use a whatwg label - See
                        https://encoding.spec.whatwg.org/#concept-encoding-get
                        (default to utf-8)
  -v,--version          Show version
```

### Example

```
# To read a file in windows-1252 (aka iso-8859-1, aka cp1252, aka latin1)
$ coda-rs --json -e latin1 FILE.CD2
```

## Features

* Parse Header
* Parse old balance (1)
* Parse movement record (2.1)
* Parse movement record (2.2)
* Parse movement record (2.3)
* Specify encoding for reading (default to utf-8)
* Parse information record (3.1)
* Parse information record (3.2)
* Parse information record (3.3)
* Parse free communication (4)
* Parse new balance (8)
* Parse trailer record (9)
* Generate JSON file
* Support account number and currency code (see 7.5 of spec)
* Trim text
* Load multiple files
* Sort by file reference

### TODO

* Support bigdecimal
* Support Structured/Unstructured communication
* JSON in array
* User friendly : List headers / oldbalance / newbalance
* List movements
* Create db (sqllite?)
* Handle globalisation
* Check the file is a valid coda file
* Add Enum for reason (2.2 - 113)
* Add code documentation
