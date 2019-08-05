// wengwengweng

attribute vec3 pos;
attribute vec3 normal;

// varying vec2 v_uv;

uniform mat4 model;
uniform mat4 view;
uniform mat4 proj;

###REPLACE###

void main() {

	vec3 world_pos = (model * vec4(pos, 1.0)).xyz;

// 	v_uv = uv;
	gl_Position = proj * view * vec4(world_pos, 1.0);

}

