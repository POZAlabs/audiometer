# audiometer

Calculate audio level meter.

## Meters

### LUFS

Loudness measurement based on ITU-R BS.1770-4

- **Integrated**: Global loudness across the entire audio
- **Momentary**: Instantaneous loudness in 100ms intervals

### RMS

Exponential Moving Average based measurement with AES17 correction

- Smooths squared sample values using an exponential moving average with 150ms decay time
- Applies +3dB AES17 correction factor
- Not standard Root Mean Square (square root of the mean of squared values over a fixed window)

### Peak

Measures the maximum absolute value of normalized samples across all channels
