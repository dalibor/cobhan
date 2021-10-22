#!/bin/sh

case $(uname -s) in
"Darwin")
    echo "Skipping gccgo install due to Darwin/macOS"
    ;;
*)
    sudo apt-get install -y gccgo-10
    sudo ln -s /usr/bin/gccgo-10 /usr/bin/gccgo
    ;;
esac


