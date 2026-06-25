# PB utility apps for development and debugging

## App Remote Debugging

Adjust, copy and launch `app-gdbserver.app` to launch an app through gdbserver for remote debugging.

Corresponding developer VSCode config:

```json
"configurations": [
    {
        "type": "lldb",
        "request": "custom",
        "name": "remote debug <app>",
        "targetCreateCommands": [
            "target create ${workspaceFolder}/target/armv7-unknown-linux-gnueabi/debug/<app>"
        ],
        "processCreateCommands": [
            "gdb-remote <pb-ip>:10003"
        ],
        "preLaunchTask": "build <app>"
    },
]
```

## Send/Receive Apps

Copy and start `app-receiver.app` on the pocketbook device, then launch:

```bash
./app-sender.sh <app-binary> <remote-app-name> <remote-ip>
```

## SSH
If you want to ssh to your device, send applications to it and listen for stdin messages,
then you have 2 ways:
1. Via KOReader (no need to root your device)
2. By setting up your own SSH Dropbear Server on the device (the device must be rooted)

### KOReader as bypass for SSH connection

[KOReader](https://github.com/koreader/koreader/releases) apart from being a third party read
it also contains some networking functionality, namely, it allows for establishing SSH connection with the device.
There is no need to root your device.

**NOTE:** Such ssh connection DOES NOTE give you root permissions.
Nevertheless, it is sufficient to deploy your app as following (it goes with no saying that before you have to register your public key):

```bash
scp -P 2222 -o HostKeyAlgorithms=+ssh-rsa target/armv7-unknown-linux-gnueabi/release/inkview-slint-demo reader@<device-ip>:/mnt/ext1/applications/application.app.stage && \
ssh -p 2222 -o HostKeyAlgorithms=+ssh-rsa reader@<device-ip> \
'sh -c "killall application.app; mv /mnt/ext1/applications/application.app.stage /mnt/ext1/applications/application.app; /mnt/ext1/applications/application.app"'
```

### Custom SSH Dropbear Server

The device must be rooted (see [here](https://github.com/ezdiy/pbjb)).

Copy and launch `ssh-dropbear-2468.app` on the pocketbook device.
Then it should be possible to connect to it through an ssh client.

The provided dropbear ssh server only accepts certain key algorithms,
a configuration can look like this:

```
Host pocketbook
  HostName <device-ip>
  Port 2468
  User root
  HostKeyAlgorithms +ssh-rsa
  PubKeyAcceptedAlgorithms +ssh-rsa
  PubkeyAuthentication=no
  StrictHostKeyChecking=no
```

Example:
```bash
scp -o HostKeyAlgorithms=+ssh-rsa target/armv7-unknown-linux-gnueabi/release/inkview-slint-demo root@<device-ip>:/mnt/ext1/applications/application.app.stage && \
ssh -o HostKeyAlgorithms=+ssh-rsa root@<device-ip> \
'sh -c "killall application.app; mv /mnt/ext1/applications/application.app.stage /mnt/ext1/applications/application.app; /mnt/ext1/applications/application.app"'
```
