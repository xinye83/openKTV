use crate::api::artist::ArtistRequest;
use crate::api::QueryParams;
use crate::model::artist::Artist;
use crate::repo::{create_pagination_query, DBRepository};

impl DBRepository {
    pub async fn insert_artist(&self, name: &String, region: &Option<String>) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("\
INSERT INTO artist (name, region)
VALUES (?, ?)
")
            .bind(name)
            .bind(region)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_id())
    }

    pub async fn query_artists(&self, request: ArtistRequest, query: &QueryParams) -> Result<Vec<Artist>, sqlx::Error> {
        let mut where_causes: Vec<(&str, String)> = Vec::new();
        if let Some(name) = request.name {
            where_causes.push(("name", name))
        }
        if let Some(region) = request.region {
            where_causes.push(("region", region))
        }
        let where_cause_str = where_causes.iter()
            .map(|it| format!("{} LIKE '%{}%'", it.0, it.1))
            .collect::<Vec<_>>()
            .join(" AND ");

        let query_str = format!("SELECT a.name AS artist_name, a.* FROM artist a WHERE {} {}", where_cause_str, create_pagination_query(query));
        let result = sqlx::query_as::<_, Artist>(query_str.as_str())
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }
}