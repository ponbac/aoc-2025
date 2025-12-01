# Use `just work 01` to work on the specific binary for a specific day's problems
work day:
    cargo watch -q -x "run -p day-{{day}}"
work-release day:
    cargo watch -q -x "run -p day-{{day}} --release"
# cargo install cargo-nextest --locked
test day:
    cargo watch -q -x "nextest run -p day-{{day}}"
new day:
    cargo new day-{{day}}
    just input {{day}}
run day:
    RUSTFLAGS='-C target-cpu=native' cargo run -p day-{{day}} --release

# You can find SESSION by using Chrome tools:
# 1) Go to https://adventofcode.com/2022/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 5) Click https://adventofcode.com under "Cookies"
# 6) Grab the value for session. Fill it into your .env file
# 
# example .env:
#
# ```
# SESSION=PASTE_COOKIE_VALUE_HERE
# ```
#
# get the input for a day's puzzle
input day:
    cargo +nightly -Zscript scripts/get-aoc-input.rs --day day-{{day}} --current-working-directory {{justfile_directory()}}
