# Kanshig

Kanshig is a program for generating and updating [Kanshi][1] configs from the current state of
your windows, based on what your wayland window manager (currently [niri][2]) reports. It
is a Rust-based TUI application.

## Kanshi

Kanshi is a tool for enabling/disabling monitor setups based on the available displays. E.g. when your laptop
is unplugged, you see just your laptop monitor. But when you plug your laptop into your dock which also has
two external displays, your laptop screen turns off. Kanshi makes this possible by listening for display
changes in wayland, and enabling/disabling available displays based on your kanshi config file when it finds
a matching setup. The config consists of display definitions (unique to each display you have), and then
collections of defined displays with indications whether they should be enabled, or not, when exactly all
of the defined displays in the collection are "matched". A kanshi config file looks like this:

```toml
output "LG Electronics LG ULTRAGEAR 112NTKFD6717" {
 mode 2560x1440@119.998
 position 0,1
 scale 1.25
 alias $HOME_0
}

output "Lenovo Group Limited B140UAN02.7  Unknown" {
 mode 1920x1200@60.000
 scale 1
 alias $INTERNAL
}

profile undocked {
 output $INTERNAL enable
}

profile home_dock {
 output $INTERNAL disable
 output $HOME_0 enable
}
```

## niri

[niri][2] is a wayland-based window manager written in Rust. It is contrasted against other wayland managers like [Hyprland] or [Sway] because
of it's "infinite scroll" tiling paradigm that adds new windows as fully vertical tiles that "stack" to the right. You can combine/split
windows in all of the typical ways you'd expect of a tiling WM, as well. It also features an 'Overview' mode that shows a zoomed-out
display of all workspaces (a workspace is a collection of windows). You can name workspaces as well as have dynamic ones. niri always
keeps an "empty" workspace at the bottom of the list, that gets added as you fill up new workspaces.

A user can issue commands to niri directly on the command-line and get back feedback about which displays, or "outputs" are connected at any given time.

```bash
$ niri msg outputs
Output "LG Electronics LG ULTRAGEAR 112NTBKD6701" (DP-8)
  Current mode: 2560x1440 @ 119.998 Hz (preferred)
  Variable refresh rate: not supported
  Physical size: 700x390 mm
  Logical position: 0, 0
  Logical size: 2048x1152
  Scale: 1.25
  Transform: normal
  Available modes:
    2560x1440@119.998 (current, preferred)
    3840x2160@60.000
    3840x2160@59.940
    3840x2160@50.000
    3840x2160@30.000
    3840x2160@29.970
    3840x2160@25.000
    3840x2160@24.000
    3840x2160@23.976
    2560x1440@143.933
    2560x1440@59.951
    1920x1080@120.000
    1920x1080@119.880
    1920x1080@60.000
    1920x1080@60.000
    1920x1080@59.940
    1920x1080@50.000
    1280x720@60.000
    1280x720@59.940
    1280x720@50.000
    1024x768@60.004
    800x600@60.317
    720x576@50.000
    720x480@60.000
    720x480@59.940
    640x480@60.000
    640x480@59.940

Output "Lenovo Group Limited B140UAN02.7  Unknown" (eDP-1)
  Disabled
  Variable refresh rate: supported, disabled
  Physical size: 300x190 mm
  Available modes:
    1920x1200@60.000 (preferred)
    1920x1080@60.000
    1600x1200@60.000
    1680x1050@60.000
    1280x1024@60.000
    1440x900@60.000
    1280x800@60.000
    1280x720@60.000
    1024x768@60.000
    800x600@60.000
    640x480@60.000
```

This output can also be provided in JSON format:

```json
$ niri msg --json outputs
{"DP-8":{"name":"DP-8","make":"LG Electronics","model":"LG ULTRAGEAR","serial":"112NTBKD6701","physical_size":[700,390],"modes":[{"width":2560,"height":1440,"refresh_rate":119998,"is_preferred":true},{"width":3840,"height":216
0,"refresh_rate":60000,"is_preferred":false},{"width":3840,"height":2160,"refresh_rate":59940,"is_preferred":false},{"width":3840,"height":2160,"refresh_rate":50000,"is_preferred":false},{"width":3840,"height":2160,"refresh_ra
te":30000,"is_preferred":false},{"width":3840,"height":2160,"refresh_rate":29970,"is_preferred":false},{"width":3840,"height":2160,"refresh_rate":25000,"is_preferred":false},{"width":3840,"height":2160,"refresh_rate":24000,"is
_preferred":false},{"width":3840,"height":2160,"refresh_rate":23976,"is_preferred":false},{"width":2560,"height":1440,"refresh_rate":143933,"is_preferred":false},{"width":2560,"height":1440,"refresh_rate":59951,"is_preferred":
false},{"width":1920,"height":1080,"refresh_rate":120000,"is_preferred":false},{"width":1920,"height":1080,"refresh_rate":119880,"is_preferred":false},{"width":1920,"height":1080,"refresh_rate":60000,"is_preferred":false},{"wi
dth":1920,"height":1080,"refresh_rate":60000,"is_preferred":false},{"width":1920,"height":1080,"refresh_rate":59940,"is_preferred":false},{"width":1920,"height":1080,"refresh_rate":50000,"is_preferred":false},{"width":1280,"he
ight":720,"refresh_rate":60000,"is_preferred":false},{"width":1280,"height":720,"refresh_rate":59940,"is_preferred":false},{"width":1280,"height":720,"refresh_rate":50000,"is_preferred":false},{"width":1024,"height":768,"refre
sh_rate":60004,"is_preferred":false},{"width":800,"height":600,"refresh_rate":60317,"is_preferred":false},{"width":720,"height":576,"refresh_rate":50000,"is_preferred":false},{"width":720,"height":480,"refresh_rate":60000,"is_
preferred":false},{"width":720,"height":480,"refresh_rate":59940,"is_preferred":false},{"width":640,"height":480,"refresh_rate":60000,"is_preferred":false},{"width":640,"height":480,"refresh_rate":59940,"is_preferred":false}],
"current_mode":0,"is_custom_mode":false,"vrr_supported":false,"vrr_enabled":false,"logical":{"x":0,"y":0,"width":2048,"height":1152,"scale":1.25,"transform":"Normal"}},"eDP-1":{"name":"eDP-1","make":"Lenovo Group Limited","mod
el":"B140UAN02.7 ","serial":null,"physical_size":[300,190],"modes":[{"width":1920,"height":1200,"refresh_rate":60000,"is_preferred":true},{"width":1920,"height":1080,"refresh_rate":60000,"is_preferred":false},{"width":1600,"he
ight":1200,"refresh_rate":60000,"is_preferred":false},{"width":1680,"height":1050,"refresh_rate":60000,"is_preferred":false},{"width":1280,"height":1024,"refresh_rate":60000,"is_preferred":false},{"width":1440,"height":900,"re
fresh_rate":60000,"is_preferred":false},{"width":1280,"height":800,"refresh_rate":60000,"is_preferred":false},{"width":1280,"height":720,"refresh_rate":60000,"is_preferred":false},{"width":1024,"height":768,"refresh_rate":6000
0,"is_preferred":false},{"width":800,"height":600,"refresh_rate":60000,"is_preferred":false},{"width":640,"height":480,"refresh_rate":60000,"is_preferred":false}],"current_mode":null,"is_custom_mode":false,"vrr_supported":true
,"vrr_enabled":false,"logical":null}
```
