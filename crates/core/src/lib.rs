use serde::Serialize;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::types::SurrealValue;

pub mod prelude;

pub type SurrealClient = Surreal<Any>;

pub trait Model
where
    Self: Serialize + for<'de> serde::Deserialize<'de> + Sized + SurrealValue,
{
    const TABLE_NAME: &'static str;
    #[cfg(feature = "utoipa")]
    type Data: Serialize + SurrealValue + 'static;
    type Input: Serialize + SurrealValue + 'static;
    fn table_name(&self) -> &'static str;
    #[cfg(feature = "utoipa")]
    fn into_data(self) -> Self::Data;
    fn objects(client: &SurrealClient) -> Objects<'_, Self, Self::Input> {
        Objects::new(client)
    }
}

pub struct Objects<'c, T, I> {
    client: &'c SurrealClient,
    _model: std::marker::PhantomData<T>,
    _data: std::marker::PhantomData<I>,
}

impl<'c, T, I> Objects<'c, T, I>
where
    T: Model + SurrealValue,
    I: Serialize + SurrealValue + 'static,
{
    pub fn new(client: &'c SurrealClient) -> Self {
        Objects {
            client,
            _model: std::marker::PhantomData,
            _data: std::marker::PhantomData,
        }
    }

    pub async fn create(&self, data: I) -> surrealdb::Result<Option<T>> {
        self.client.create(T::TABLE_NAME).content(data).await
    }

    pub async fn create_with_id(&self, id: String, data: I) -> surrealdb::Result<Option<T>> {
        self.client.create((T::TABLE_NAME, id)).content(data).await
    }

    pub async fn create_many(&self, data: Vec<I>) -> surrealdb::Result<Option<Vec<T>>> {
        self.client.create(T::TABLE_NAME).content(data).await
    }

    pub async fn get_by_id(&self, id: &str) -> surrealdb::Result<Option<T>> {
        self.client.select((T::TABLE_NAME, id)).await
    }

    pub async fn update(&self, id: &str, data: I) -> surrealdb::Result<Option<T>> {
        self.client.update((T::TABLE_NAME, id)).content(data).await
    }

    pub async fn upsert(&self, id: &str, data: I) -> surrealdb::Result<Option<T>> {
        self.client.upsert((T::TABLE_NAME, id)).content(data).await
    }

    pub async fn delete(&self, id: &str) -> surrealdb::Result<Option<T>> {
        self.client.delete((T::TABLE_NAME, id)).await
    }

    pub async fn all(&self) -> surrealdb::Result<Vec<T>> {
        self.client.select(T::TABLE_NAME).await
    }

    pub async fn drop(&self) -> surrealdb::Result<Vec<T>> {
        self.client.delete(T::TABLE_NAME).await
    }
}
