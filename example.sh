#!/bin/sh

itmp="./tmp.file.ipc"

gencsv(){
	echo timestamp,severity,status,body
	echo 2025-10-20T23:44:59.012345Z,INFO,200,apt update done
	echo 2025-10-21T23:44:59.012345Z,INFO,500,apt update failure
	echo 2025-10-22T23:44:59.012345Z,INFO,500,apt update failure
	echo 2025-10-23T23:44:59.012345Z,INFO,500,apt update failure
	echo 2025-10-24T23:44:59.012345Z,INFO,500,apt update failure
}

geninput(){
	echo generating input file...

	which csv2arrow2ipc | fgrep -q csv2arrow2ipc || exec sh -c '
		echo csv2arrow2ipc missing.
		echo see github.com/takanoriyanagitani/go-csv2arrow2ipc to install it.
		exit 1
	'

	gencsv |
		csv2arrow2ipc |
		cat > "${itmp}"
}

head_native(){
	arrow-file-to-stream "${itmp}" |
		./arrow-ipc-stream-head 4
}

head_wasi(){
	arrow-file-to-stream "${itmp}" |
		wazero run ./arrow-ipc-stream-head.wasm 4
}

run_native(){
	head_native |
		arrow-cat
}

run_wasi(){
	head_wasi |
		arrow-cat
}

test -f "${itmp}" || geninput

which wazero | fgrep -q wazero && run_wasi
which wazero | fgrep -q wazero || run_native
