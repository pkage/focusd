# focusd 
## a rust daemon for time-controlled modification of /etc/hosts

This daemon modifies `/etc/hosts` to add a block of sites to DNS sinkhole for a user-defined period of time.

## installation

```
$ git clone git@github.com:pkage/focusd && cd focusd
$ cargo build --release
$ cp target/release/focusd ~/.local/bin
$ mkdir -p ~/.config/focusd && cp focus.toml ~/.config/focusd/focus.toml
```

Additionally, if you would like to avoid running the focusd daemon as root, you
can set the permissions of `/etc/hosts` like so:

```
$ sudo chmod +a /etc/hosts
```

or

```
$ sudo chown `whoami` /etc/hosts
```

...depending on what your system supports.

## usage

### configuration

By default, focusd uses a config file located in `~/.config/focus.toml`:

```toml
version     = '0.0.2'
hosts_file  = '/etc/hosts'
socket_file = '/tmp/focusd.sock'
pid_file    = '/tmp/focusd.pid'

blocked = [
    'news.ycombinator.com',
    'youtube.com',
    'facebook.com',
    'reddit.com',
    'twitter.com',
    'xkcd.com',
    'smbc-comics.com',
    'netflix.com'
]
```

### daemon

Run the daemon with:

```
$ focusd daemon
```

If you have not configured /etc/hosts access, you will need to run this as sudo.

### client

Run the client with any of:

```
$ focusd ping
$ focusd start 1h30m
$ focusd remaining
$ focusd halt
```

`focusd start` accepts times such as:

 - `1h30m`
 - `30m`
 - `15m20s`
 - `60s`

according to the regular expression:

```regex
^([0-9]+h)?([0-9]+m)?([0-9]+s)?$
```

`focusd remaining` accepts two optional flags:

short | long           | description
--    | --             | --
`-n`  | `--nodistract` | omits the seconds counter
`-r`  | `--raw`        | shows the time in seconds without formatting

### troubleshooting

In the event of trouble, either of these should help:

```
$ focusd halt
$ killall focusd
$ focusd cleanup
```

### polybar integration

Place this into your polybar config:

```ini
[module/focusd]
type = custom/script
exec = focusd remaining -n
interval = 1

# optional
format-prefix = "<font>"
format-margin = 4
```

