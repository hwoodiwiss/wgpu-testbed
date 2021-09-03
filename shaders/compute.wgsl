

struct ModelVertex {
	x: f32, x: f32, z: f32,
	uv: f32, uw: f32,
	nx: f32, ny: f32, nz: f32,
	tx: f32, ty: f32, tz: f32,
	bx: f32, by: f32, bz: f32,
}

[[stage(compute)]]
fn main([[builtin(global_invocation_id)]] global_ix: vec3<u32>) {
	let vert_idx = global_ix.x;
	let result = calcTangentBitangent(vert_idx);
	dst_vertices[vert_idx] = result;
}


fn calcTangentBitangent(vert_idx: u32) -> ModelVertex {
	var curr_vert = src_verts[vert_idx];

	var tangent: vec3<f32>;
	var bitangent: vec3<f32>;
	var tris_included: u32;

	for(var i: u32 = 0; i < numIndices; i += 3) {
		let idx0 = indices[i];
		let idx1 = indices[i + 1];
		let idx2 = indices[i + 2];

		if(idx0 == vert_idx || idx1 == vert_idx || idx2 == vert_idx) {
			let vert0 = src_verts[idx0];
			let vert1 = src_verts[idx1];
			let vert2 = src_verts[idx2]

			let pos0 = getPos(vert0);
			let pos1 = getPos(vert1);
			let pos2 = getPos(vert2);

			let tex0 = getUV(vert0);
			let tex1 = getUV(vert1);
			let tex2 = getUV(vert2);

			let delta_pos1 = pos1 - pos0;
			let delta_pos2 = pos2 - pos0;

			let delta_uv1 = tex1 - text0;
			let delta_uv2 = tex2 - tex0;

			let r: f32 = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
			tangent += (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
            bitangent += (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * r; 
            trianglesIncluded += 1;
		}
	}

	if(trianglesIncluded > 0) {
		tangent /= trianglesIncluded;
        bitangent /= trianglesIncluded;
        tangent = normalize(tangent);
        bitangent = normalize(bitangent);
	}

	curr_vert.tx = tangent.x;
	curr_vert.ty = tangent.y;
	curr_vert.tz = tangent.z;

	curr_vert.bx = bitangent.x;
    curr_vert.by = bitangent.y;
    curr_vert.bz = bitangent.z;

	return curr_vert;
}

fn getPos(vert: ModelVertex) -> vec3<f32> {
	return vec3<f32> {
		vert.x,
		vert.y,
		vert.z
	};
}

fn getUV(vert: ModelVertex) -> vec2<f32> {
	return vec2<f32> {
		vert.uv,
		vert.uw,
	};
}