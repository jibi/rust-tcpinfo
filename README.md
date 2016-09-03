rust-tcpstat
=============

rust + LDPRELOAD == this unreadable mess

#### Usage

```
$ cargo build --release
$ LD_PRELOAD=$(pwd)/target/release/libtcpstat.so curl google.com -o /dev/null -s
```
