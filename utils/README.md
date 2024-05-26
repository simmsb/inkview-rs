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

## SSH Dropbear Server

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
