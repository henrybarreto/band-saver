![BandwidthAID logo](logo.png)
# BandwidthAID
GTK application to limit network speed to save bandwidth.

## How it works?
It uses the `wondershaper` script to set speed limit to a selected network interface.

## How to use?
> TO USE IT, YOU **NEED** `wondershaper` SCRIPT INSTALLED.

To compile, you'll need the `cargo` tool installed.

### Check 
Checks if wondershaper is installed
```sh
make check
```

### Build 
```sh
make build
```

### Run 
```sh
make run 
```

### Bin 
Runs the binary
```sh
sudo make bin 
```
