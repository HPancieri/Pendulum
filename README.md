# Pendulum Simulator

Simple pendulum simulator written in Rust.<br>
Modeled accordingly to the equation:

```math
F = P \cdot \sin(\theta)
m \cdot a = m \cdot g \cdot \sin(\theta)
a = g \cdot \sin(\theta)
\alpha \cdot r = g \cdot \sin(\theta)
\alpha = \frac{g \cdot \sin(\theta)}{r}
```

Where $\alpha$ is the angular acceleration, g is gravity, $\theta$ is the angle, and r is the length of the pendulum.