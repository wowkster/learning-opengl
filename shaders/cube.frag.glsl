#version 330 core
in vec3 Normal;
in vec3 FragPos;

uniform vec3 lightPos;  
uniform vec3 viewPos;

uniform vec3 lightColor;
uniform vec3 objectColor;

void main()
{
    // Calculate Ambient Lighting
    float ambientStrength = 0.1;
    vec3 ambient = ambientStrength * lightColor;

    // Calculate Diffuse Lighting
    vec3 norm = normalize(Normal);
    vec3 lightDirection = normalize(lightPos - FragPos);

    float difference = max(dot(norm, lightDirection), 0.0);
    vec3 diffuse = difference * lightColor;

    // Calculate the Specular Refections
    float specularStrength = 0.5;
    float specularShininess = 32;

    vec3 viewDirection = normalize(viewPos - FragPos);
    vec3 reflectDirection = reflect(-lightDirection, norm);

    float spec = pow(max(dot(viewDirection, reflectDirection), 0.0), specularShininess);
    vec3 specular = specularStrength * spec * lightColor;

    // Put it all together
    vec3 result = (ambient + diffuse + specular) * objectColor;
    gl_FragColor = vec4(result, 1.0f);
}