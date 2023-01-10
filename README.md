# Pendulum Simulator

Simple pendulum simulator written in Rust.<br>
Modeled accordingly to the equation:

$F = P \cdot \sin(\theta)$<br>
$m \cdot a = m \cdot g \cdot \sin(\theta)$<br>
$a = g \cdot \sin(\theta)$<br>
$\alpha \cdot r = g \cdot \sin(\theta)$<br>
$\alpha = \frac{g \cdot \sin(\theta)}{r}$<br>

Where $\alpha$ is the angular acceleration, g is gravity, $\theta$ is the angle, and r is the length of the pendulum.
