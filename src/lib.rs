use smart_contract::log;
use smart_contract::payload::Parameters;
use smart_contract_macros::smart_contract;
use serde_json::json;
// use uuid::Uuid;

use indradb::{Datastore, MemoryDatastore, Transaction, VertexQueryExt}; //::{MemoryDatastore};

// #[derive(Serialize, Deserialize)]
struct Contract {
    store: MemoryDatastore,
}

#[smart_contract]
impl Contract {
    pub fn init(params: &mut Parameters) -> Self {
        Self {
            store: MemoryDatastore::default(),
        }
    }

    pub fn get_schemas(&mut self, _params: &mut Parameters) -> Result<(), String> {
        Ok(())
    }

    pub fn insert_vertex(&mut self, params: &mut Parameters) -> Result<(), String> {
        let trans = self.store.transaction().unwrap();

        let vertex_type: String = params.read();
        let vertex_t = indradb::Type::new(vertex_type.clone()).unwrap();
        let outbound_v = indradb::Vertex::with_id(
            uuid::Uuid::new_v3(&uuid::Uuid::NAMESPACE_DNS, &params.transaction_id),
            vertex_t.clone(),
        );
        trans.create_vertex(&outbound_v).unwrap();
        Ok(())
    }

    pub fn get_vertices_by_type(&mut self, params: &mut Parameters) -> Result<(), String> {
        let trans = self.store.transaction().unwrap();
        let vertex_type: String = params.read();
        let type_filter = indradb::Type::new(vertex_type.clone()).unwrap();
        let range = trans
            .get_vertices(indradb::RangeVertexQuery::new(std::u32::MAX).t(type_filter))
            .unwrap();
        for vertex in range.into_iter() {
            log(&vertex.id.to_string())
        }
        Ok(())
    }

    pub fn set_value(&mut self, params: &mut Parameters) -> Result<(), String>  {
        let trans = self.store.transaction().unwrap();
        let id_str: String = params.read();
        let uuid = uuid::Uuid::parse_str(&id_str).unwrap();
        let props_str: String = params.read();
        let props: serde_json::Value = serde_json::from_str(&props_str).unwrap();
        // let id = uuid::Uuid::new_v3(&uuid::Uuid::NAMESPACE_DNS, &params.transaction_id),
        // let range = trans.get_vertices(indradb::SpecificVertexQuery::single(uuid)).unwrap();
        // let vertex = range.get(0).unwrap();
        trans.set_vertex_properties(indradb::SpecificVertexQuery::single(uuid).property("props"), &props);
        Ok(())
    }

    pub fn get_value(&mut self, params: &mut Parameters) -> Result<(), String>  {
        let trans = self.store.transaction().unwrap();
        let id_str: String = params.read();
        let uuid = uuid::Uuid::parse_str(&id_str).unwrap();
        // let id = uuid::Uuid::new_v3(&uuid::Uuid::NAMESPACE_DNS, &params.transaction_id),
        // let range = trans.get_vertices(indradb::SpecificVertexQuery::single(uuid)).unwrap();
        // let vertex = range.get(0).unwrap();
        let res = trans.get_vertex_properties(indradb::SpecificVertexQuery::single(uuid).property("props")).unwrap();
        log(&serde_json::to_string(&res.get(0).unwrap().value).unwrap());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
