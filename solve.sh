cargo build --release --quiet
if [ $? -eq 0 ]; then
  time cat inputs/$1.txt | ./target/release/day$1 $@
fi