use serde::Serialize;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
//Rust edtion 2018+ 就已经弃用了extern crate了

pub mod prelude;

/// 这里应该使用Any 来处理surreal的所有连接类型；Client只能支持Remote；我们是否应该创建自定义类型来托管SurrealDB的连接？
pub type SurrealClient = Surreal<Any>;

pub trait Model
where
    Self: Serialize + for<'de> serde::Deserialize<'de> + Sized,
{
    const TABLE_NAME: &'static str;
    type Data: Serialize + 'static;
    fn table_name(&self) -> &'static str;
    fn into_data(self) -> Self::Data;
    fn objects(client: &SurrealClient) -> Objects<'_, Self, Self::Data> {
        Objects::new(client)
    }
}

pub struct Objects<'c, T, D> {
    client: &'c SurrealClient,
    _model: std::marker::PhantomData<T>,
    _data: std::marker::PhantomData<D>,
}

impl<'c, T, D> Objects<'c, T, D>
where
    T: Model,
    D: Serialize + 'static,
{
    pub fn new(client: &'c SurrealClient) -> Self {
        Objects {
            client,
            _model: std::marker::PhantomData,
            _data: std::marker::PhantomData,
        }
    }

    pub async fn create(client: &'c SurrealClient, data: D) -> surrealdb::Result<Option<T>> {
        client.create(T::TABLE_NAME).content(data).await
    }

    pub async fn create_with_id(&self, id: String, data: D) -> surrealdb::Result<Option<T>> {
        self.client.create((T::TABLE_NAME, id)).content(data).await
    }

    pub async fn create_many(&self, data: Vec<D>) -> surrealdb::Result<Option<Vec<T>>> {
        self.client.create(T::TABLE_NAME).content(data).await
    }

    pub async fn get_by_id(&self, id: &str) -> surrealdb::Result<Option<T>> {
        self.client.select((T::TABLE_NAME, id)).await
    }

    pub async fn update(&self, id: &str, data: D) -> surrealdb::Result<Option<T>> {
        self.client.update((T::TABLE_NAME, id)).content(data).await
    }

    pub async fn upsert(&self, id: &str, data: D) -> surrealdb::Result<Option<T>> {
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
