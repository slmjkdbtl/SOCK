// wengwengweng

vec4 frag() {
	return v_color * u_color * texture(u_tex, v_uv);
}

