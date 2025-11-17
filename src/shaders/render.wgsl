struct VertexOutput {
  @builtin(position) pos: vec4f,
  @location(0) color: vec4f,
}

@vertex
fn vertexMain(@location(0) pos: vec2f, @location(1) color: vec4f) -> VertexOutput {
  var output: VertexOutput;
  output.pos = vec4f(pos, 0, 1);
  output.color = color;
  return output;
}

@fragment
fn fragmentMain(input: VertexOutput) -> @location(0) vec4f {
  return input.color;
}
