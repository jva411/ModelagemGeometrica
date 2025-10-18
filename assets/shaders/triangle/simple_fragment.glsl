#version 330 core

struct Material {
  vec3 diffuse;
  vec3 specular;
  float shininess;
};

in vec3 normal;
in vec3 fragmentPosition;

uniform Material material;
uniform vec3 cameraPosition;

out vec4 FragColor;

void main() {
  vec3 diffuse = vec3(0.8, 0.8, 0.8) * material.diffuse;
  vec3 specular = vec3(0.1, 0.1, 0.1) * material.specular;

  FragColor = vec4(diffuse + specular, 1.0);
}
