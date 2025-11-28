#!/usr/bin/env nu

cargo zigbuild --release --bin inkview-slint-demo --target armv7-unknown-linux-gnueabi.2.23 --features sdk-6-8
scp -o HostKeyAlgorithms=+ssh-rsa target/armv7-unknown-linux-gnueabi/release/inkview-slint-demo root@192.168.1.113:/mnt/ext1/applications/slint.app.stage
ssh -o HostKeyAlgorithms=+ssh-rsa root@192.168.1.113 'sh -c "killall slint.app; mv /mnt/ext1/applications/slint.app.stage /mnt/ext1/applications/slint.app; /mnt/ext1/applications/slint.app"'
