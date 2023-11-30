# To run day-02 part_1: just run 2 1
run DAY PART:
	cargo run -p `printf "day-%02d" {{DAY}}` --bin part_{{PART}}

# To test day-01 part_2: just test 1 2
test DAY PART="":
	cargo test -p `printf "day-%02d" {{DAY}}` part_{{PART}}
