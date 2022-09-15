use crate::{
	fusion::{
		node::GenNodeInfo,
		node::{Node, NodeError},
		spatial::Spatial,
	},
	push_to_vec,
	values::{Quat, Vec3, QUAT_IDENTITY, VEC3_ONE, VEC3_ZERO},
};
use anyhow::Result;
use std::ops::Deref;

use super::Field;

pub struct BoxField {
	pub field: Field,
}
#[buildstructor::buildstructor]
impl<'a> BoxField {
	#[builder(entry = "builder")]
	pub fn create(
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		size: Option<Vec3>,
	) -> Result<Self, NodeError> {
		Ok(BoxField {
			field: Field {
				spatial: Spatial {
					node: generate_node!(
						GenNodeInfo {
							client: spatial_parent.node.client.clone(),
							parent_path: "/field",
							interface_path: "/field",
							interface_method: "createBoxField"
						},
						spatial_parent.node.get_path(),
						position.unwrap_or(VEC3_ZERO),
						rotation.unwrap_or(QUAT_IDENTITY),
						size.unwrap_or(VEC3_ONE)
					),
				},
			},
		})
	}
}
impl Deref for BoxField {
	type Target = Field;

	fn deref(&self) -> &Self::Target {
		&self.field
	}
}

#[tokio::test]
async fn fusion_box_field() {
	use crate::fusion::client::Client;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");
	let box_field = BoxField::builder()
		.spatial_parent(client.get_root())
		.build()
		.expect("Unable to make box field");

	let client_captured = client.clone();
	let distance = box_field
		.field
		.distance(
			client_captured.get_root(),
			mint::Vector3::from([0_f32, 2_f32, 0_f32]),
		)
		.await
		.expect("Unable to get box field distance");
	assert_eq!(distance, 1_f32);

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}