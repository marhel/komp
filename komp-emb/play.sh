# [0]: Picoprobe (CMSIS-DAP) (VID: 2e8a, PID: 000c, Serial: E66118604B235F26, CmsisDap)
export PROBE_ID="2e8a:000c:E66118604B235F26"
MY_PATH="$(dirname -- "${BASH_SOURCE[0]}")"
MY_PATH="$(cd -- "$MY_PATH" && pwd)" # absolutized and normalized

cd $MY_PATH/player
cargo run -- --probe $PROBE_ID
