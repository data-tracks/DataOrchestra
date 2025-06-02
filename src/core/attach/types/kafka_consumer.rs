use serde::{Deserialize, Serialize};

use crate::core::types::data::{NodeData, DockerData};
use crate::shared::ToInternal;
use crate::interface::{general::General, object::ExtObject};
use crate::core::adapters::ContainerBuilder;
use crate::core::object::Object;
use crate::core::attach::attach_types::ToObject;

// The Kafka consumer type. Is an attachable object capable of consuming data from kafka topic(s)
// and sending them further through a http request
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KafkaConsumer {
    // Topics the consumer reads from
    pub topics: Vec<String>,
    // External Object type to allow for the configuration of the consumer
    pub object: Box<ExtObject>
}

impl ToObject for KafkaConsumer {
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
                "attachables/kafka_consumer/", 
                "kafka_consumer/", 
                "kafka_consumer/start.sh", 
                None)
            );

        object.node_data = general.node_data
            .into_iter()
            .map(|item| item.to_internal())
            .collect::<Vec<NodeData>>();
        
        object.docker_container_builder.get_or_insert(ContainerBuilder::new());

        if let Some(builder) = object.docker_container_builder.as_mut() {
            builder
                .try_set_name("KafkaConsumer");
        }

        object
    }
}
