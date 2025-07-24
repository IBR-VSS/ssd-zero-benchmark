scp-vm:
    scp target/debug/zeroing-bench llzero-vm:
    ssh llzero-vm 'patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2' zeroing-bench

scp-baremetal:
    scp target/debug/zeroing-bench debian-local:benchmark
    scp ./fio/seqread.fio debian-local:seqread.fio
    ssh debian-local 'patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2' benchmark

scp-orwa:
    scp target/debug/zeroing-bench orwa.ibr:local/llzero/benchmark
    scp ./fio/seqread.fio orwa.ibr:local/llzero/
    ssh orwa.ibr 'patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2 local/llzero/benchmark'
