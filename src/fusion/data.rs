use std::sync::Weak;

use super::{
	client::Client,
	field::Field,
	node::{GenNodeInfo, Node, NodeError},
	spatial::Spatial,
	values::{Quat, Vec3},
};
use crate::{
	flex,
	fusion::values::{QUAT_IDENTITY, VEC3_ZERO},
};

pub struct PulseReceiver {
	pub spatial: Spatial,
	// pub field: &'a Field,
}

#[buildstructor::buildstructor]
impl<'a> PulseReceiver {
	#[builder(entry = "builder")]
	pub async fn create(
		client: Weak<Client>,
		spatial_parent: &'a Spatial,
		position: Option<Vec3>,
		rotation: Option<Quat>,
		field: &'a Field,
	) -> Result<Self, NodeError> {
		Ok(PulseReceiver {
			spatial: Spatial {
				node: generate_node!(
					GenNodeInfo {
						client: client.clone(),
						parent_path: "/data/receiver",
						interface_path: "/data",
						interface_method: "createPulseReceiver"
					},
					spatial_parent.node.get_path(),
					position.unwrap_or(VEC3_ZERO),
					rotation.unwrap_or(QUAT_IDENTITY),
					field.spatial.node.get_path()
				),
			},
		})
	}
}

#[tokio::test]
async fn fusion_pulse_receiver() {
	use std::sync::Arc;
	let (client, event_loop) = Client::connect_with_async_loop()
		.await
		.expect("Couldn't connect");

	let field = super::field::SphereField::builder()
		.client(Arc::downgrade(&client))
		.spatial_parent(client.get_root())
		.radius(0.1)
		.build()
		.await
		.unwrap();
	let _pulse_receiver = PulseReceiver::builder()
		.client(Arc::downgrade(&client))
		.spatial_parent(client.get_root())
		.field(&field.field)
		.build()
		.await
		.unwrap();

	tokio::select! {
		biased;
		_ = tokio::signal::ctrl_c() => (),
		_ = event_loop => (),
	};
}
