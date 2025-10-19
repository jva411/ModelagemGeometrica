#version 330 core


layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in mat4 aModel;

uniform mat4 baseModel;
uniform mat4 view;
uniform mat4 projection;

out vec3 normal;
out vec3 fragmentPosition;

void main() {
  mat4 model = baseModel * aModel;
  vec4 worldPos = model * vec4(aPos, 1.0);
  gl_Position = projection * view * worldPos;
  fragmentPosition = vec3(worldPos);
  normal = normalize(mat3(transpose(inverse(model))) * aNormal);
}
