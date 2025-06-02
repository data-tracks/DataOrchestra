use serde::{Deserialize, Serialize};

use crate::core::adapters::ContainerBuilder;
use crate::interface::general::General;
use crate::core::types::{data::NodeData, DockerData};
use crate::core::object::Object;
use crate::core::attach::attach_types::ToObject;
use crate::interface::object::ExtObject;
use crate::shared::ToInternal;

// The Kafka consumer type. Is an attachable object capable of consuming data from kafka topic(s)
// and sending them further through a http request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaProducer {
    // Topics the producer writes to
    pub topics: Vec<String>,
    // External Object type to allow for the configuration of the producer
    pub object: Box<ExtObject>
}

impl ToObject for KafkaProducer {
    fn to_object(self, general: &General) -> Object {
        // Clone due to the general struct later also being used to parse the actual object where
        // the attach object is attached to
        let general = general.clone();

        let mut object = self.object.to_internal();

        if let Some(node) = general.node {
            object.node = Some(node.to_internal());
        }

        if let Some(ansible) = general.ansible {
            object.ansible = ansible;        
        }

        object.docker_data = general.docker_data
            .into_iter()
            .map(|item| item.to_internal())
            .collect::<Vec<DockerData>>();

        object.docker_data.push(DockerData::new
            (
                "", 
                "attachables/kafka_producer/", 
                "kafka_producer/", 
                "kafka_producer/start.sh", 
                None)
            );

        object.node_data = general.node_data
            .into_iter()
            .map(|item| {
                item.to_internal()
            })
            .collect::<Vec<NodeData>>();
        
        object.docker_container_builder.get_or_insert(ContainerBuilder::new());

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_set_name("KafkaProducer");
        }

        object
    }
}
