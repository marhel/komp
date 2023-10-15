MY_PATH="$(dirname -- "${BASH_SOURCE[0]}")"
MY_PATH="$(cd -- "$MY_PATH" && pwd)" # absolutized and normalized
alias drive=$MY_PATH/drive.sh
alias play=$MY_PATH/play.sh