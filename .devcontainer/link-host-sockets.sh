#!/bin/sh
# Links the host X11 / PulseAudio sockets to the fixed paths used by the dev image
# (DISPLAY=:0, PULSE_SERVER=unix:/tmp/pulse-native), whatever their names on the host:
#   - X11: /run/host-x11/X<N> (WSLg: X0, native Linux: X<N> from the host DISPLAY=:<N>)
#   - PulseAudio: /run/host-pulse/PulseServer (WSLg) or /run/host-pulse/native (native Linux)
# Runs at each container start (dev container postStartCommand, dev image entrypoint).

for socket in /run/host-x11/X*; do
  if [ -S "$socket" ]; then
    ln -sfn "$socket" /tmp/.X11-unix/X0
    break
  fi
done

for socket in /run/host-pulse/PulseServer /run/host-pulse/native; do
  if [ -S "$socket" ]; then
    ln -sfn "$socket" /tmp/pulse-native
    break
  fi
done

# Missing sockets only disable the GUI / audio: never block the container start
exit 0
