# This is a MOXA NPort 51xx CLI tool.
This tool requires you to use Nport Admin to preset the device to a TCP Server and the Serial interface to what you need it to be able to speak to your machine.

It compiles using the rust chain.
use rustup to install (https://rustup.rs/) and then use cargo to build.

# Usage:
```
./moxa-cli -h
moxa-cli 0.1.0
Moxa NPort 51xx CLI

IP address is required

Usage: moxa-cli [OPTIONS]

Options:
  -f, --file
  -n, --ip
  -p, --port                       [default: 4001]
  -P, --packet-size                [default: 50]
  -i, --interval                   [default: 0]
  -h, --help                       Print help
  -V, --version                    Print version
```
# Example:
```
./moxa-cli -f test.txt -n 192.168.1.201 -S 50 -i 100

OR

cat test.txt | ./moxa-cli -n 192.168.1.201 -S 50 -i 100
```

# Video.
[![Watch Videos](https://i9.ytimg.com/vi_webp/Xyyfbxs5dKQ/mq2.webp?sqp=CNi0hqEG-oaymwEmCMACELQB8quKqQMa8AEB-AGICYAC0AWKAgwIABABGBEgYChyMA8=&rs=AOn4CLD8-JVS6CZX1FiZuZzuDRXy2P-UOA)](https://www.youtube.com/watch?v=Xyyfbxs5dKQ)