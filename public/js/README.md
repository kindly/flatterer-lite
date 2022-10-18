# Flatterer WASM frontend

[Full Documentation](http://flatterer.opendata.coop/)

## Introduction

An opinionated JSON to CSV/XLSX/SQLITE/PARQUET converter which tries to make a useful relational output for data analysis.

## Rationale

When receiving a JSON file where the structure is deeply nested or not well specified, it is hard to determine what the data contains. Also, even after knowing the JSON structure, it requires a lot of time to work out how to flatten the JSON into a relational structure to do data analysis on and to be part of a data pipeline. 

Flatterer aims to be the first tool to go to when faced with the above problem.  It may not be the tool that you end up using to flatten the JSON in your data pipeline, as hand written flattening may be required, but it could be.  It has many benefits over most hand written approaches:

* It is fast, written in rust but with python bindings for ease of use.  It can be 10x faster than hand written python flattening.
* Memory efficient.  Uses a custom streaming JSON parser to mean that long list of objects nested with the JSON will be streamed, so not much data needs to be loaded into memory at once.
* Fast memory efficient output to CSV/XLSX/SQLITE/PARQUET
* Uses best practice that has been learnt from flattening JSON countless times, such as generating keys to link one-to-many tables to their parents.

