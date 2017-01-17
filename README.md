# yggdrasil

A server for watching your plants and stuff.

Currently setup as a listener and the arduino just spams down some serial channel. Hopefully will move to yggdrasil polling its clients.


# How do?!

On linux you provide the USB file descriptor, which probably looks something like this:

`yggdrasil /dev/ttyUSB0`

On windows you'll have to specify the COM interface, like so:

`yggdrasil COM1`

The server will then start up on either a port specificed with `--port=NUMBER` or `8508` by default. Any problems should result in a helpful error message.


# Problems

## Serial Communication

Serial comms with the Arduino currently assumes system endianess is little endian, this will need to be fixed before we can deploy onto a big endian system.

Serial comms currently uses packed structures which supposedly can be bad on some architectures, if it's a problem on RPi the struct should be padded and copy to a byte array before transmission.
