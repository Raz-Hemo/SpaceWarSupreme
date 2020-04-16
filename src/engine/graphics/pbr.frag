#version 450
precision highp float;
// Based on https://learnopengl.com/PBR/Theory
layout(location=0) in vec3 v_worldpos;
layout(location=1) in vec2 v_tex;
layout(location=2) in vec3 v_tangent;
layout(location=3) in vec3 v_norm;
layout(location=4) in flat uint v_ent;

layout(location=0) out vec4 fragColor;
layout(location=1) out uint fragPick;

// material parameters
layout(binding=0) uniform sampler2D albedo;
layout(binding=1) uniform sampler2D metallic_rough; // metallic is the B channel, roughness is G
layout(binding=2) uniform sampler2D ao;
layout(binding=3) uniform sampler2D normal_map;

// lights
struct PointLight {
    vec3 pos;
    vec3 color;
};
#define NUM_LIGHTS 2
uniform PointLight pointlights[NUM_LIGHTS];
uniform vec3 cameraPosition;
uniform float exposure;

const float PI = 3.14159265359;
// ----------------------------------------------------------------------------
float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / denom;
}
// ----------------------------------------------------------------------------
float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}
// ----------------------------------------------------------------------------
float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}
// ----------------------------------------------------------------------------
vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}
// ----------------------------------------------------------------------------
void main()
{
    // Tangent space calculations
    vec3 T = normalize(v_tangent);
    vec3 N = normalize(v_norm);
    vec3 B = cross(N, T);
    mat3 TBN = mat3(T, B, N);
    N = N;//TBN * (texture(normal_map, v_tex).xyz * 2.0 - 1.0);

    vec3 V = normalize(cameraPosition - v_worldpos);
    vec3 R = reflect(-V, N); 
    vec3 f_albedo = texture(albedo, v_tex).xyz;
    float f_roughness = texture(metallic_rough, v_tex).g;
    float f_metallic = texture(metallic_rough, v_tex).b;

    // calculate reflectance at normal incidence; if dia-electric (like plastic) use F0 
    // of 0.04 and if it's a metal, use their albedo color as F0 (metallic workflow)    
    vec3 F0 = vec3(0.04); 
    F0 = mix(F0, f_albedo, f_metallic);

    // reflectance equation
    vec3 Lo = vec3(0.0);
    for(int i = 0; i < NUM_LIGHTS; ++i) 
    {
        // calculate per-light radiance
        vec3 L = normalize(pointlights[i].pos - v_worldpos);
        vec3 H = normalize(V + L);
        float distance = length(pointlights[i].pos - v_worldpos);
        vec3 radiance = pointlights[i].color / (distance * distance);

        // Cook-Torrance BRDF
        float NDF = DistributionGGX(N, H, f_roughness);
        float G   = GeometrySmith(N, V, L, f_roughness);
        vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);
           
        vec3 nominator    = NDF * G * F; 
        float denominator = 4.0 * max(dot(V, N), 0.0) * max(dot(L, N), 0.0) + 0.001f; // 0.001 to prevent divide by zero.
        vec3 brdf = nominator / denominator;
        
        // kS is equal to Fresnel
        vec3 kS = F;
        // for energy conservation, the diffuse and specular light can't
        // be above 1.0 (unless the surface emits light); to preserve this
        // relationship the diffuse component (kD) should equal 1.0 - kS.
        vec3 kD = vec3(1.0) - kS;
        // multiply kD by the inverse metalness such that only non-metals 
        // have diffuse lighting, or a linear blend if partly metal (pure metals
        // have no diffuse light).
        kD *= 1.0 - f_metallic;

        // scale light by NdotL
        float NdotL = max(dot(N, L), 0.0);

        // add to outgoing radiance Lo
        Lo += (kD * f_albedo / PI + brdf) * radiance * NdotL;  // note that we already multiplied the BRDF by the Fresnel (kS) so we won't multiply by kS again
    }   
    
    // ambient lighting (note that the next IBL tutorial will replace 
    // this ambient lighting with environment lighting).
    vec3 ambient = vec3(0.03) * f_albedo * texture(ao, v_tex).xxx;

    vec3 color = ambient + Lo;

    // HDR tonemapping
    color = color / (color + vec3(1.0));
    // gamma correct
    color = pow(color, vec3(1.0/exposure)); 

    fragColor = vec4(color, 1.0);
    fragPick = v_ent;
}