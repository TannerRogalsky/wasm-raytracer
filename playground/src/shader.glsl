varying vec2 v_uv;

#ifdef VERTEX
attribute vec2 position;
attribute vec2 uv;

void main() {
    v_uv = uv;
    vec2 pos = (position - 0.5) * 2.0;
    pos.y *= -1.0;
    gl_Position = vec4(pos, 0.0, 1.0);
}
#endif

#ifdef FRAGMENT
uniform sampler2D tex0;

void main() {
    fragColor = Texel(tex0, v_uv);
}
#endif