// AIFramework/src/context/context_graph_manager.rs

use crate::context::forms::{Form, FormRelation, ContextGraph};
use std::sync::{Arc, RwLock};

pub struct ContextGraphManager {
    context_graph: Arc<RwLock<ContextGraph>>, // Thread-safe access to the graph
}

impl ContextGraphManager {
    pub fn new() -> Self {
        ContextGraphManager {
            context_graph: Arc::new(RwLock::new(ContextGraph::new())),
        }
    }

    pub fn get_graph(&self) -> Arc<RwLock<ContextGraph>> {
        self.context_graph.clone()
    }

    /// Integrates new forms and relations into the ContextGraph (Division & Collection).
    pub fn assimilate_forms(&self, new_forms: Vec<Form>, new_relations: Vec<FormRelation>) {
        let mut graph = self.context_graph.write().unwrap(); // Acquire write lock

        for form in new_forms {
            // Phase 3: Division - Determine hierarchical placement (simplified)
            // A real implementation would involve similarity search in Form.representation
            // or symbolic reasoning to find super/sub-forms.
            if form.description.contains("generalized") && !form.id.contains("GEN_") {
                // Example: Create a "super-form" if not already present
                let super_form_id = format!("GEN_{}", form.id);
                if !graph.forms.contains_key(&super_form_id) {
                    graph.add_form(Form {
                        id: super_form_id.clone(),
                        description: format!("Generalized {}", form.description),
                        representation: form.representation.clone(),
                        meta_properties: form.meta_properties.clone(),
                        examples_count: 1,
                        ethical_score: form.ethical_score,
                    });
                }
                graph.add_relation(FormRelation {
                    source_form_id: form.id.clone(),
                    target_form_id: super_form_id,
                    relation_type: "IS_A".to_string(),
                    strength: 0.9,
                });
            }
            graph.add_form(form);
        }

        // Phase 3: Collection - Add new relationships
        for relation in new_relations {
            graph.add_relation(relation);
        }

        println!("Context Graph updated with new forms and relations.");
    }

    // Additional methods for querying, persisting, loading the graph
}
