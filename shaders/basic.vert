#version 410

in vec3 position;
in vec3 normal;

smooth out vec3 _normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;


void main() {
    // learnopengl.com tells me to do it all in reverse order, idfk why
    gl_Position = projection * view * model * vec4(position, 1);

    // Magic maths on the normal
    _normal = transpose(inverse(mat3(model))) * normal;

}