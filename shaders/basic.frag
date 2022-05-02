#version 450

smooth in vec3 _normal;

out vec3 color;

uniform vec3 light;

void main() {
    // Returns a -1 <-> 1 depending on the normals angle compared to the light vector
    // We normalise it to 0 <-> 1
    float dif = (dot(_normal, light) + 1) / 2;

    float a = 0;

    // We then divide it into bands, to cel shade it.
    // Apparently using ifs is bad? But I don't know a better way to do this without using textures
    // like in Zelda windwaker
    if (dif > 0.8) a = 1.0;
    else if (dif > 0.5) a = 0.8;
    else if (dif > 0.3) a = 0.5;
    else a = 0.3;

    color = vec3(a,a,a);
}