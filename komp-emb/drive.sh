# [1]: Picoprobe (CMSIS-DAP) (VID: 2e8a, PID: 000c, Serial: E6614103E75ABC30, CmsisDap)
export PROBE_ID="2e8a:000c:E6614103E75ABC30"
MY_PATH="$(dirname -- "${BASH_SOURCE[0]}")"
MY_PATH="$(cd -- "$MY_PATH" && pwd)" # absolutized and normalized

cd $MY_PATH/driver
cargo run -- --probe $PROBE_ID
