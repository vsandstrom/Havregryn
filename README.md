# Havregryn

Havregryn is a granular delay and texture synthesizer. The name is swedish for grains of oats, from which you can make havregrynsgröt, oatmeal.

<img width="700" alt="Skärmavbild 2024-07-17 kl  16 25 01" src="https://github.com/user-attachments/assets/c5a7b58d-7ea3-4ea0-85ba-0316ec32b7c0">

### Controls
- position<br>sets the position in the recorded buffer from where to start the next grain playback.
  
- jitter<br>applies some random offset to the position value, setting the playback position ahead or behind by a factor of $x * bufferlength$.
  
- duration<br>sets the duration of each grain in seconds.
  
- trigger<br>sets the interval between each grain.
  
- rate<br>sets the playback rate of a grain, a value between $(-1.0, 1.0)$, which means at $0.0$ rate the playback will be silent, and at $-1.0$ the grains play in reverse.
  
- mod freq<br>sets the frequency of the underlying LFO that modulates the playback rate.
  
- mod amount<br>sets the factor of how much modulation that will be used.
  
- random<br>changes the trigger mode from a static duration to a randomized duration between $(0.0, 2.0) * trigger\ interval$

- sample<br>resets the record buffer and starts recording new input. 

All values are sampled at the creation of a new grain, after that point it is out of your control.

___current issue, the sample button should be momentary but this has not yet been implemented, you have to leave it checked for the recording AND playback.___

## Building

After installing [Rust](https://rustup.rs/), you can compile Havregryn as follows:

```shell
cargo xtask bundle havregryn --release
```

This will compile a VST3 for your platform. 
