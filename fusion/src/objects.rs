#![allow(async_fn_in_trait)]

use crate::{client::Client, spatial::SpatialRef};
use stardust_xr::{
	schemas::{
		dbus::{PlaySpaceProxy, SpatialRefProxy},
		zbus::Connection,
	},
	values::Vector2,
};
use std::sync::Arc;

async fn connection() -> Option<Connection> {
	Connection::session().await.ok()
}
pub trait SpatialRefProxyExt {
	async fn import(&self, stardust_client: &Arc<Client>) -> Option<SpatialRef>;
}
impl SpatialRefProxyExt for SpatialRefProxy<'_> {
	async fn import(&self, stardust_client: &Arc<Client>) -> Option<SpatialRef> {
		let uid = self.uid().await.ok()?;
		SpatialRef::import(stardust_client, uid).await.ok()
	}
}

pub async fn hmd(client: &Arc<Client>) -> Option<SpatialRef> {
	SpatialRefProxy::new(
		&connection().await?,
		"org.stardustxr.HMD",
		"/org/stardustxr/HMD",
	)
	.await
	.ok()?
	.import(client)
	.await
}

pub struct PlaySpace {
	pub spatial: SpatialRef,
	pub bounds_polygon: Vec<Vector2<f32>>,
}
pub async fn play_space(client: &Arc<Client>) -> Option<PlaySpace> {
	let connection = connection().await?;
	let spatial_proxy = SpatialRefProxy::new(
		&connection,
		"org.stardustxr.PlaySpace",
		"/org/stardustxr/PlaySpace",
	)
	.await
	.ok()?;
	let spatial = spatial_proxy.import(client).await?;
	let play_space_proxy = PlaySpaceProxy::new(&connection).await.ok()?;
	let bounds_polygon = play_space_proxy.bounds().await.ok()?;
	Some(PlaySpace {
		spatial,
		bounds_polygon: bounds_polygon
			.into_iter()
			.map(|(x, y)| [x as f32, y as f32].into())
			.collect(),
	})
}