use atspi::Interface;
use atspi_proxies::{
	accessible::AccessibleProxy, action::ActionProxy, application::ApplicationProxy,
	collection::CollectionProxy, component::ComponentProxy, document::DocumentProxy,
	editable_text::EditableTextProxy, hyperlink::HyperlinkProxy, hypertext::HypertextProxy,
	image::ImageProxy, selection::SelectionProxy, table::TableProxy,
	table_cell::TableCellProxy, text::TextProxy, value::ValueProxy,
};
use std::future::Future;
use zbus::{
	names::InterfaceName, proxy::Builder as ProxyBuilder, proxy::CacheProperties,
	proxy::Defaults as ProxyDefault, proxy::ProxyImpl, Error, Proxy,
};

#[allow(clippy::module_name_repetitions)]
pub trait Convertable<'a> {
	type Error: std::error::Error;

	/// Creates an [`AccessibleProxy`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation of.
	/// Generally, it fails if the accessible item does not implement to accessible interface.
	/// This shouldn't be possible, but this function may fail for other reasons.
	/// For example, to convert a [`zbus::Proxy`] into a [`AccessibleProxy`], it may fail to create the new [`atspi_proxies::accessible::AccessibleProxy`].
	fn to_accessible(
		&self,
	) -> impl Future<Output = Result<AccessibleProxy, Self::Error>> + Send;
	/// Creates an [`ActionProxy`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to action interface.
	fn to_action(&self) -> impl Future<Output = Result<ActionProxy, Self::Error>> + Send;

	/// Creates an [`ApplicationProxy`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// Generally, it fails if the accessible item does not implement to application interface.
	fn to_application(
		&self,
	) -> impl Future<Output = Result<ApplicationProxy, Self::Error>> + Send;

	/// Creates an [`CollectionProxy`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// it fails if the accessible item does not implement to collection interface.
	fn to_collection(
		&self,
	) -> impl Future<Output = Result<CollectionProxy, Self::Error>> + Send;

	/// Creates an [`ComponentProxy`] from the existing accessible item.
	/// # Errors
	///
	/// This may fail based on the implementation.
	/// it fails if the accessible item does not implement to component interface.
	fn to_component(&self) -> impl Future<Output = Result<ComponentProxy, Self::Error>> + Send;
	fn to_document(&self) -> impl Future<Output = Result<DocumentProxy, Self::Error>> + Send;
	fn to_hypertext(&self) -> impl Future<Output = Result<HypertextProxy, Self::Error>> + Send;
	fn to_hyperlink(&self) -> impl Future<Output = Result<HyperlinkProxy, Self::Error>> + Send;
	fn to_image(&self) -> impl Future<Output = Result<ImageProxy, Self::Error>> + Send;
	fn to_selection(&self) -> impl Future<Output = Result<SelectionProxy, Self::Error>> + Send;
	fn to_table(&self) -> impl Future<Output = Result<TableProxy, Self::Error>> + Send;
	fn to_table_cell(&self)
		-> impl Future<Output = Result<TableCellProxy, Self::Error>> + Send;
	fn to_text(&self) -> impl Future<Output = Result<TextProxy, Self::Error>> + Send;
	fn to_editable_text(
		&self,
	) -> impl Future<Output = Result<EditableTextProxy, Self::Error>> + Send;
	fn to_value(&self) -> impl Future<Output = Result<ValueProxy, Self::Error>> + Send;
}

#[inline]
async fn convert_to_new_type<
	'a,
	'b,
	T: From<Proxy<'b>> + ProxyDefault,
	U: ProxyImpl<'a> + ProxyDefault,
>(
	from: &U,
) -> zbus::Result<T> {
	let from = from.inner();

	// first thing is first, we need to creat an accessible to query the interfaces.
	let accessible = AccessibleProxy::builder(from.connection())
		.destination(from.destination())?
		.cache_properties(CacheProperties::No)
		.path(from.path())?
		.build()
		.await?;
	// if the interface we're trying to convert to is not available as an interface; this can be problematic because the interface we're passing in could potentially be different from what we're converting to.
	let new_interface_name_ref: &InterfaceName = <T as ProxyDefault>::INTERFACE
		.as_ref()
		.ok_or(Error::InterfaceNotFound)?;
	let new_interface_name: Interface = serde_plain::from_str(new_interface_name_ref)
		.map_err(|_| Error::InterfaceNotFound)?;
	if !accessible.get_interfaces().await?.contains(new_interface_name) {
		return Err(Error::InterfaceNotFound);
	}
	// otherwise, make a new Proxy with the related type.
	let path = from.path().to_owned();
	let dest = from.destination().to_owned();

	ProxyBuilder::<T>::new(from.connection())
		.interface(
			<T as ProxyDefault>::INTERFACE
				.as_ref()
				.ok_or(Error::InterfaceNotFound)?,
		)?
		.destination(dest)?
		.cache_properties(CacheProperties::No)
		.path(path)?
		.build()
		.await
}

impl<'a, T: ProxyImpl<'a> + ProxyDefault + Sync> Convertable<'a> for T {
	type Error = zbus::Error;
	/* no guard due to assumption it is always possible */
	async fn to_accessible(&self) -> zbus::Result<AccessibleProxy> {
		convert_to_new_type(self).await
	}
	async fn to_action(&self) -> zbus::Result<ActionProxy> {
		convert_to_new_type(self).await
	}
	async fn to_application(&self) -> zbus::Result<ApplicationProxy> {
		convert_to_new_type(self).await
	}
	async fn to_collection(&self) -> zbus::Result<CollectionProxy> {
		convert_to_new_type(self).await
	}
	async fn to_component(&self) -> zbus::Result<ComponentProxy> {
		convert_to_new_type(self).await
	}
	async fn to_document(&self) -> zbus::Result<DocumentProxy> {
		convert_to_new_type(self).await
	}
	async fn to_hypertext(&self) -> zbus::Result<HypertextProxy> {
		convert_to_new_type(self).await
	}
	async fn to_hyperlink(&self) -> zbus::Result<HyperlinkProxy> {
		convert_to_new_type(self).await
	}
	async fn to_image(&self) -> zbus::Result<ImageProxy> {
		convert_to_new_type(self).await
	}
	async fn to_selection(&self) -> zbus::Result<SelectionProxy> {
		convert_to_new_type(self).await
	}
	async fn to_table(&self) -> zbus::Result<TableProxy> {
		convert_to_new_type(self).await
	}
	async fn to_table_cell(&self) -> zbus::Result<TableCellProxy> {
		convert_to_new_type(self).await
	}
	async fn to_text(&self) -> zbus::Result<TextProxy> {
		convert_to_new_type(self).await
	}
	async fn to_editable_text(&self) -> zbus::Result<EditableTextProxy> {
		convert_to_new_type(self).await
	}
	async fn to_value(&self) -> zbus::Result<ValueProxy> {
		convert_to_new_type(self).await
	}
}
