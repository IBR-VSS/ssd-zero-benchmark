scp-vm:
    scp target/debug/zeroing-bench llzero-vm:
    ssh llzero-vm 'patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2' zeroing-bench

scp-baremetal:
    scp target/debug/zeroing-bench debian-local:benchmark
    scp ./fio/seqread.fio debian-local:seqread.fio
    ssh debian-local 'patchelf --set-interpreter /lib64/ld-linux-x86-64.so.2' benchmark
