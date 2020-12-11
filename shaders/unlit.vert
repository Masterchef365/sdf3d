
#version 450
#extension GL_ARB_separate_shader_objects : enable
#extension GL_EXT_multiview : require

// IO stuff
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

layout(location = 0) out vec3 fragColor;

// SDF stuff
struct SDF {
    float dist;
    vec3 color;
};

SDF sphere(vec3 pos, vec3 origin, vec3 color, float radius) {
    return SDF(
        distance(pos, origin) - radius,
        color
    );
}

SDF cube(vec3 pos, vec3 origin, vec3 color, float side) {
    vec3 pt = pos - origin;
    return SDF(
        distance(pt, clamp(vec3(-side), pt, vec3(side))),
        color
    );
}

SDF sdf_min(SDF a, SDF b) {
    if (a.dist < b.dist) {
        return a;
    } else {
        return b;
    }
}

SDF scene(vec3 pos) {
    return sdf_min(
        sphere(pos, vec3(0.168, 0.088, (1.800)), vec3(0.721,0.995,0.123), 0.8),
        cube(pos, vec3(0., 0.460, (1.776)), vec3(0.995,0.467,0.002), 0.5)
    );
}

void main() {
    gl_Position = vec4(inPosition, 1.);
    fragColor = inPosition;
}

