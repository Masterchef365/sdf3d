#version 450
#extension GL_ARB_separate_shader_objects : enable

layout(location = 0) in vec3 fragPos;

layout(binding = 1) uniform Animation {
    float value;
} anim;

layout(location = 0) out vec4 outColor;

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
    vec2 st = vec2(fragPos.x, -fragPos.y);

    vec3 initial_ray = vec3(st, 1.);
    vec3 unit_ray = normalize(initial_ray);
	vec3 color = vec3(0.);
    
    vec3 pos = initial_ray;
    for (int i = 0; i < 100; i++) {
        SDF hit = scene(pos);
        if (hit.dist < 0.001) {
            color = hit.color;
            break;
        }
        pos += unit_ray * hit.dist;
    }

    outColor = vec4(color, 1.0);
}
