# Pendulum Simulator

Simple pendulum simulator written in Rust.<br>
Modeled accordingly to the equation:

![equation](https://latex.codecogs.com/gif.image?%5Cdpi%7B110%7DF%20=%20P%20%5Ccdot%20%5Csin(%5Ctheta)%5C%5C)<br>
![equation](https://latex.codecogs.com/gif.image?%5Cdpi%7B110%7Dm%20%5Ccdot%20a%20=%20m%20%5Ccdot%20g%20%5Ccdot%20%5Csin(%5Ctheta))<br>
![equation](https://latex.codecogs.com/gif.image?%5Cdpi%7B110%7Da%20=%20g%20%5Ccdot%20%5Csin(%5Ctheta))<br>
![equation](https://latex.codecogs.com/gif.image?%5Cdpi%7B110%7D%5Calpha%20%5Ccdot%20r%20=%20g%20%5Ccdot%20%5Csin(%5Ctheta))<br>
![equation](https://latex.codecogs.com/gif.image?%5Cdpi%7B110%7D%5Calpha%20=%20%5Cfrac%7Bg%20%5Ccdot%20%5Csin(%5Ctheta)%7D%7Br%7D)<br>

Where ⍺ is the angular acceleration, g is gravity, θ is the angle, and r is the length of the pendulum.