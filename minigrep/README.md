## Simple grep cli

#### Usage:
`cargo run {text_search} {filename}` to run case sensitive search
`cargo run -- -i {text_search} {filename}` to run case insensitive search

#### Example:
`cargo run software sample.txt`
`cargo run -- -i pagemaker sample.txt`