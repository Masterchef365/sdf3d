
#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_EXT_multiview : require

layout(binding = 0) uniform CameraUbo {
    mat4 matrix[2];
} cam;

layout(binding = 1) uniform Animation {
    float value;
} anim;

layout(push_constant) uniform Model {
    mat4 matrix;
} model;

layout(location = 0) in vec3 inPosition;

layout(location = 0) out vec3 fragPos;

void main() {
    gl_Position = vec4(inPosition, 1.);
    fragPos = vec3(inPosition.x, -inPosition.y, inPosition.z);
}

