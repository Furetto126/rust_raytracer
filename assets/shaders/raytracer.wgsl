@group(0) @binding(0) var texture: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1) var<storage, read> camera_position: vec3<f32>;
@group(0) @binding(2) var<storage, read> camera_direction: vec3<f32>;
@group(0) @binding(3) var<storage, read> resolution: vec2<f32>;
@group(0) @binding(4) var<storage, read> aspect_ratio: f32;
@group(0) @binding(5) var<storage, read> inverse_view_matrix: mat4x4<f32>;
@group(0) @binding(6) var<storage, read> spheres: array<Sphere>;

// textureLoad to read from texture
// let value: vec4<f32> = textureLoad(texture, fragCoord + vec2<i32>(offset_x, offset_y)); 

@compute @workgroup_size(8, 8, 1)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>, @builtin(num_workgroups) num_workgroups: vec3<u32>) {
    let fragCoord = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    textureStore(texture, fragCoord, color);
}

const PI = 3.141592653589793238462643;

struct Ray {
    origin: vec3<f32>,
    direction: vec3<f32>
}

struct Sphere {
    position: vec3<f32>,
    radius: f32
}

fn get_environment_light(ray: Ray) -> vec3<f32> {
    let skyColorHorizon = vec3<f32>(0.9, 1.0, 1.0);
    let skyColorZenith = vec3<f32>(0.37, 0.47, 0.81);
    let groundColor = vec3<f32>(0.41, 0.39, 0.37);

    let skyGradientT = smoothstep(0.0, 1.0, ray.direction.y);
    let skyGradient = mix(skyColorHorizon, skyColorZenith, skyGradientT);

    let groundToSkyT = smoothstep(-0.001, 0.0, -ray.direction.y);
    return mix(groundColor, skyGradient, groundToSkyT);
}

fn sphere_intersection(ray: Ray, index: i32) -> vec3<f32> {
    let sphere = spheres[index];
    let epsilon = 0.00001;

    let oc = ray.origin - sphere.position;
    let a = dot(ray.direction, ray.direction);
    let b = 2.0 * dot(oc, ray.direction);
    let c = dot(oc, oc) - sphere.radius * sphere.radius;

    var disc = b*b - 4.0*a*c;
    if (disc < 0.0) {
        return get_environment_light(ray);
    }

    disc = sqrt(disc);
    let sol1 = (-b + disc) / (2.0 * a);
    let sol2 = (-b - disc) / (2.0 * a);
    let solMin = min(sol1, sol2);

    if (solMin > epsilon) {
        return vec3<f32>(1.0);
    }
    return get_environment_light(ray);
}

@compute @workgroup_size(8, 8, 1)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let fragCoord = vec2<f32>(location);

    let fov = PI / 2.0;
    let tan_fov = 1.0 / tan(fov * 0.5 * PI / 180.0);

    let ndc = vec2<f32>(
        ((2.0 * fragCoord.x - resolution.x) / resolution.x) * aspect_ratio,
        (2.0 * fragCoord.y - resolution.y) / resolution.y
    );

    let ray_target = (inverse_view_matrix * vec4<f32>(ndc, tan_fov, 1.0)).xyz;
    let ray_direction = normalize(ray_target - camera_position);
    var ray = Ray(camera_position, ray_direction);

    var color = vec3<f32>(0.0, 0.0, 0.0);
    for (var i: i32 = 0; i < i32(arrayLength(&spheres)); i = i + 1) {
        color = sphere_intersection(ray, i);
    }

    storageBarrier();
    textureStore(texture, location, vec4<f32>(color, 1.0));
}