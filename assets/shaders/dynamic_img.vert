#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
uniform vec2 TexCoordShift;
uniform vec2 PosCoordShift;
uniform mat2 PosRotation;

out vec2 TexCoord;

void main()
{
  vec2 r = (aPos + PosCoordShift); //  * PosRotation;
  gl_Position = vec4(r, 0.0, 1.0);
	TexCoord = aTexCoord + TexCoordShift;
}
