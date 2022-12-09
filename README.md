# ds4led-iced
Did you ever want to change the led color of your dualshock 4 in your linux machine with a gui tool? No? Fuck you, here is a gui tool to do it

This is basically a wrapper for [ds4led](https://gist.github.com/jacobmischka/7f96f3fddf93dbe21db2) written in [iced](https://iced.rs/) <br>
In the future i want to do everything within rust, it just needs to edit files

## Usage
```
git clone https://github.com/Steccah/ds4led-iced.git
cd ds4led-iced
sudo mv src/ds4led /usr/bin
cargo run --release
```
You'll find the executable in target

I hate iced there is no documentation
