// Run: %dxc -T ps_6_0 -E main

cbuffer Data {
    float    gScalars[1];
    float4   gVecs[2];
    float2x3 gMats[1];
}

struct T {
    float    scalars[1];
    float4   vecs[2];
    float2x3 mats[1];
};

float4 main() : SV_Target {
    T t;

// CHECK:        [[gscalars_ptr:%\d+]] = OpAccessChain %_ptr_Uniform__arr_float_uint_1 %Data %int_0
// CHECK-NEXT:   [[gscalars_val:%\d+]] = OpLoad %_arr_float_uint_1 [[gscalars_ptr]]
// CHECK-NEXT:    [[scalars_ptr:%\d+]] = OpAccessChain %_ptr_Function__arr_float_uint_1_0 %t %int_0
// CHECK-NEXT:      [[gscalars0:%\d+]] = OpCompositeExtract %float [[gscalars_val]] 0
// CHECK-NEXT:    [[scalars_val:%\d+]] = OpCompositeConstruct %_arr_float_uint_1_0 [[gscalars0]]
// CHECK-NEXT:                           OpStore [[scalars_ptr]] [[scalars_val]]
    t.scalars = gScalars;

// CHECK-NEXT: [[gvecs_ptr:%\d+]] = OpAccessChain %_ptr_Uniform__arr_v4float_uint_2 %Data %int_1
// CHECK-NEXT: [[gvecs_val:%\d+]] = OpLoad %_arr_v4float_uint_2 [[gvecs_ptr]]
// CHECK-NEXT:  [[vecs_ptr:%\d+]] = OpAccessChain %_ptr_Function__arr_v4float_uint_2_0 %t %int_1
// CHECK-NEXT:    [[gvecs0:%\d+]] = OpCompositeExtract %v4float [[gvecs_val]] 0
// CHECK-NEXT:    [[gvecs1:%\d+]] = OpCompositeExtract %v4float [[gvecs_val]] 1
// CHECK-NEXT:  [[vecs_val:%\d+]] = OpCompositeConstruct %_arr_v4float_uint_2_0 [[gvecs0]] [[gvecs1]]
// CHECK-NEXT:                      OpStore [[vecs_ptr]] [[vecs_val]]
    t.vecs    = gVecs;

// CHECK-NEXT: [[gmats_ptr:%\d+]] = OpAccessChain %_ptr_Uniform__arr_mat2v3float_uint_1 %Data %int_2
// CHECK-NEXT: [[gmats_val:%\d+]] = OpLoad %_arr_mat2v3float_uint_1 [[gmats_ptr]]
// CHECK-NEXT:  [[mats_ptr:%\d+]] = OpAccessChain %_ptr_Function__arr_mat2v3float_uint_1_0 %t %int_2
// CHECK-NEXT:    [[gmats0:%\d+]] = OpCompositeExtract %mat2v3float [[gmats_val]] 0
// CHECK-NEXT:  [[mats_val:%\d+]] = OpCompositeConstruct %_arr_mat2v3float_uint_1_0 [[gmats0]]
// CHECK-NEXT:                      OpStore [[mats_ptr]] [[mats_val]]
    t.mats    = gMats;

    return t.vecs[1];
}
