// RUN: %dxc -E main -T ps_6_0 %s | FileCheck %s

// CHECK: "uav1", i32 0, i32 3, i32 1, i32 1, i1 true

globallycoherent RWTexture1D<float4> uav1 : register(u3);
RWBuffer<float4> uav2;

float4 main(uint2 a :  A, uint2 b : B) : SV_Target
{
  globallycoherent  RWTexture1D<float4> uav3 = uav1;
  uav3[0] = 5;
  uav1[0] = 2;
  uav2[1] = 3;
  return 0;
}
