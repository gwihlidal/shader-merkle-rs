// RUN: %dxc -E main -T gs_6_0 %s | FileCheck %s

// CHECK: InputPrimitive=patch2
// CHECK: emitStream
// CHECK: cutStream
// CHECK: i32 24}

struct GSOut {
  float2 uv : TEXCOORD0;
  float4 clr : COLOR;
  float4 pos : SV_Position;
  float3 norm[2] : NORMAL;
};

cbuffer b : register(b0) {
  float2 invViewportSize;
};

// geometry shader that outputs 3 vertices from a point
[maxvertexcount(3)]
[instance(24)]
void main(InputPatch<GSOut, 2>points, inout PointStream<GSOut> stream) {

  points[0].norm[0] = 1;
  points[0].norm[1] = 2;
  stream.Append(points[0]);

  stream.RestartStrip();
}