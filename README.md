# Minimal Weather CLI Tool

This small script gets the average temperature and total precipitation for the next "x" hours at your current location (based on your IP). 

### Requirements
- `serde`
- `reqwest`
- `serde_json`

### Build 
You can build the binary with cargo: `cargo build --release`

The binary takes 1 argument (the hours): `./weather_cli 5`
