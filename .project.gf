[gdb]
path=./rust-gdb

[commands]
Compile ice-puzzles=shell cargo b --bin ice-puzzles --profile debugging
Run ice-puzzles=file target/debugging/ice-puzzles;run&