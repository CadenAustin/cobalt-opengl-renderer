#version 330 core

uniform int highlight;
uniform vec3 ambient_color;
uniform vec3 diffuse_color;
uniform vec3 specular_color;
uniform vec3 camera_pos;
uniform vec3 light_pos;
uniform float shininess;

uniform int ambient_map_exists;
uniform int diffuse_map_exists;
uniform int specular_map_exists;
uniform sampler2D ambient_map;
uniform sampler2D diffuse_map;
uniform sampler2D specular_map;


in vec3 v_normal;
in vec2 v_tex;
in vec3 FragPos;

out vec4 FragColor;

const vec3 light_color = vec3(1., 1., 1.);

vec3 shading() {
    vec3 ambient = ambient_color * light_color;

    vec3 norm = normalize(v_normal);
    vec3 light_dir = normalize(light_pos - FragPos);
    float diff = max(dot(norm, light_dir), 0.0);
    vec3 diffuse = light_color * (diff * diffuse_color);

    if (ambient_map_exists == 1) {
        ambient = ambient * vec3(texture(ambient_map, v_tex));
    }

    if (diffuse_map_exists == 1) {
        diffuse = diffuse * vec3(texture(diffuse_map, v_tex));
    }

    vec3 view_dir = normalize(camera_pos - FragPos);
    vec3 reflectDir = reflect(-light_dir, norm);
    float spec = pow(max(dot(view_dir, reflectDir), 0.0), shininess);
    vec3 specular = light_color * (spec * specular_color);   

    return ambient + diffuse + specular;
}

void main()
{

    if (highlight == 1) {
        FragColor = vec4(shading() * .5, 1.0f);
    } else {
        FragColor = vec4(shading(), 1.0f);
    }
    
} 