# MeiliDB

[![Build Status](https://dev.azure.com/thomas0884/thomas/_apis/build/status/meilisearch.MeiliDB?branchName=master)](https://dev.azure.com/thomas0884/thomas/_build/latest?definitionId=1&branchName=master)
[![dependency status](https://deps.rs/repo/github/Kerollmops/MeiliDB/status.svg)](https://deps.rs/repo/github/Kerollmops/MeiliDB)
[![License](https://img.shields.io/github/license/Kerollmops/MeiliDB.svg)](https://github.com/Kerollmops/MeiliDB)
[![Rust 1.31+](https://img.shields.io/badge/rust-1.31+-lightgray.svg)](
https://www.rust-lang.org)

A _full-text search database_ using a key-value store internally.

## Features

- Provides [6 default ranking criteria](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-core/src/criterion/mod.rs#L95-L101) used to [bucket sort](https://en.wikipedia.org/wiki/Bucket_sort) documents
- Accepts [custom criteria](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-core/src/criterion/mod.rs#L22-L29) and can apply them in any custom order
- Support [ranged queries](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-core/src/query_builder.rs#L146), useful for paginating results
- Can [distinct](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-core/src/query_builder.rs#L68) and [filter](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-core/src/query_builder.rs#L57) returned documents based on context defined rules
- Can store complete documents or only [user schema specified fields](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/examples/movies/schema-movies.toml)
- The [default tokenizer](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-tokenizer/src/lib.rs#L99) can index latin and kanji based languages
- Returns [the matching text areas](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-core/src/lib.rs#L117-L120), useful to highlight matched words in results
- Accepts query time search config like the [searchable fields](https://github.com/meilisearch/MeiliDB/blob/3d85cbf0cfa3a3103cf1e151a75a443719cdd5d7/meilidb-core/src/query_builder.rs#L79)
- Supports run time indexing  (incremental indexing)



It uses [sled](https://github.com/spacejam/sled) as the internal key-value store. The key-value store allows us to handle updates and queries with small memory and CPU overheads. The whole ranking system is [data oriented](https://github.com/meilisearch/MeiliDB/issues/82) and provides great performances.

You can [read the deep dive](deep-dive.md) if you want more information on the engine, it describes the whole process of generating updates and handling queries or you can take a look at the [typos and ranking rules](typos-ranking-rules.md) if you want to know the default rules used to sort the documents.

We will be proud if you submit issues and pull requests. You can help to grow this project and start contributing by checking [issues tagged "good-first-issue"](https://github.com/meilisearch/MeiliDB/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22). It is a good start!

The project is only a library yet. It means that there is no binary provided yet. To get started, you can check the examples wich are made to work with the data located in the `misc/` folder.

MeiliDB will be a binary in a near future so you will be able to use it as a database out-of-the-box. We should be able to query it using a [to-be-defined](https://github.com/meilisearch/MeiliDB/issues/38) protocol. This is our current goal, [see the milestones](https://github.com/meilisearch/MeiliDB/milestones). In the end, the binary will be a bunch of network protocols and wrappers around the library - which will also be published on [crates.io](https://crates.io). Both the binary and the library will follow the same update cycle.



## Performances

With a database composed of _100 353_ documents with _352_ attributes each and _3_ of them indexed.
So more than _300 000_ fields indexed for _35 million_ stored we can handle more than _2.8k req/sec_ with an average response time of _9 ms_ on an Intel i7-7700 (8) @ 4.2GHz.

Requests are made using [wrk](https://github.com/wg/wrk) and scripted to simulate real users queries.

```
Running 10s test @ http://localhost:2230
  2 threads and 25 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     9.52ms    7.61ms  99.25ms   84.58%
    Req/Sec     1.41k   119.11     1.78k    64.50%
  28080 requests in 10.01s, 7.42MB read
Requests/sec:   2806.46
Transfer/sec:    759.17KB
```

### Notes

The default Rust allocator has recently been [changed to use the system allocator](https://github.com/rust-lang/rust/pull/51241/).
We have seen much better performances when [using jemalloc as the global allocator](https://github.com/alexcrichton/jemallocator#documentation).

## Usage and examples

You can try a little part of MeiliDB with the following commands.
It creates an index named _movies_ and insert two great Tarantino movies in it.

```bash
cargo run --release

curl -XPOST 'http://127.0.0.1:8000/movies' \
    -d '
identifier = "id"

[attributes.id]
stored = true

[attributes.title]
stored = true
indexed = true
'

curl -H 'Content-Type: application/json' \
     -XPUT 'http://127.0.0.1:8000/movies' \
     -d '{ "id": 123, "title": "Inglorious Bastards" }'

curl -H 'Content-Type: application/json' \
     -XPUT 'http://127.0.0.1:8000/movies' \
     -d '{ "id": 456, "title": "Django Unchained" }'
```

Once the database is initialized you can query it by using the following command:

```bash
curl -XGET 'http://127.0.0.1:8000/movies/search?q=inglo'
```
