#! /usr/bin/env bash

rm -r test
mkdir test
for f in {1..1000}; do
	#./target/debug/mklink.exe test/target_$f 123
	./target/release/mklink.exe test/target_$f 123
	#echo $f
done
