// AIFramework/src/context/forms.rs

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

// Represents an abstract Platonic Form
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Form {
    pub id: String, // Unique identifier (e.g., "Symmetry", "Causality", "Objectness")
    pub description: String,
    pub representation: Vec<f32>, // Latent vector, embedding, or symbolic representation
    pub meta_properties: HashMap<String, String>, // E.g., "complexity": "high", "stability": "immutable"
    pub examples_count: usize, // How many percepts contributed to this form
    pub ethical_score: f32, // Score from Ethical Filter (0-1, 1=highly ethical)
}

// Represents a relationship between two Forms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormRelation {
    pub source_form_id: String,
    pub target_form_id: String,
    pub relation_type: String, // E.g., "IS_A", "PART_OF", "CAUSES", "ANALOGOUS_TO"
    pub strength: f32, // Confidence score (0-1)
}

// The Context Graph itself
pub struct ContextGraph {
    forms: HashMap<String, Form>,
    relations: Vec<FormRelation>,
    // Potentially use a graph database or a specialized graph library (e.g., petgraph)
}

impl ContextGraph {
    pub fn new() -> Self {
        ContextGraph {
            forms: HashMap::new(),
            relations: Vec::new(),
        }
    }

    pub fn add_form(&mut self, form: Form) {
        self.forms.insert(form.id.clone(), form);
    }

    pub fn add_relation(&mut self, relation: FormRelation) {
        self.relations.push(relation);
    }

    // Example: Query for related forms
    pub fn get_related_forms(&self, form_id: &str, relation_type: Option<&str>) -> Vec<&Form> {
        let mut related = Vec::new();
        for rel in &self.relations {
            if rel.source_form_id == form_id && (relation_type.is_none() || relation_type == Some(&rel.relation_type)) {
                if let Some(form) = self.forms.get(&rel.target_form_id) {
                    related.push(form);
                }
            }
        }
        related
    }

    // More complex graph query methods would be added here
}
