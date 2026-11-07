use crate::business_knowledge_model::BusinessKnowledgeModelEvaluator;
use crate::decision::DecisionEvaluator;
use crate::decision_service::DecisionServiceEvaluator;
use crate::input_data::InputDataEvaluator;
use crate::input_data_context::InputDataContextEvaluator;
use crate::item_definition::ItemDefinitionEvaluator;
use crate::item_definition_context::ItemDefinitionContextEvaluator;
use crate::item_definition_type::{InformationItemTypes, ItemDefinitionTypeEvaluator};
use crate::model_definitions::{DefDefinitions, DefKey, Invocables};
use dsntk_common::Result;
use dsntk_feel::Name;
use dsntk_model::Definitions;
use std::cell::RefCell;

pub struct EvaluatorBuilders {
    pub input_data_evaluator: InputDataEvaluator,
    pub item_definition_evaluator: ItemDefinitionEvaluator,
    pub business_knowledge_model_evaluator: BusinessKnowledgeModelEvaluator,
    pub decision_evaluator: DecisionEvaluator,
    pub decision_service_evaluator: DecisionServiceEvaluator,
    pub invocables: Invocables,
    pub information_item_types: InformationItemTypes,
}

/// Model builder.
#[derive(Default)]
pub struct ModelBuilder {
    /// Model definitions.
    model_definitions: DefDefinitions,
    /// Input data evaluator builder.
    input_data_evaluator: InputDataEvaluator,
    /// Input data context evaluator builder.
    input_data_context_evaluator: InputDataContextEvaluator,
    /// Item definition evaluator builder.
    item_definition_evaluator: ItemDefinitionEvaluator,
    /// Item definition context evaluator builder.
    item_definition_context_evaluator: ItemDefinitionContextEvaluator,
    /// Item definition type evaluator builder.
    item_definition_type_evaluator: ItemDefinitionTypeEvaluator,
    /// Business knowledge model evaluator builder.
    business_knowledge_model_evaluator: BusinessKnowledgeModelEvaluator,
    /// Decision evaluator builder.
    decision_evaluator: DecisionEvaluator,
    /// Decision service evaluator builder.
    decision_service_evaluator: DecisionServiceEvaluator,
    /// Map of invocables indexed by invocable name.
    invocables: RefCell<Invocables>,
}

impl ModelBuilder {
    /// Adds definitions from specified model.
    pub fn add_model(&mut self, definitions: &Definitions) {
        self.model_definitions.add_model(definitions);
    }

    /// Builds a model based on model definitions.
    pub fn build(&mut self) -> Result<()> {
        self.input_data_evaluator = InputDataEvaluator::new(&self.model_definitions);
        self.input_data_context_evaluator = InputDataContextEvaluator::new(&self.model_definitions);
        self.item_definition_evaluator = ItemDefinitionEvaluator::new(&self.model_definitions)?;
        self.item_definition_context_evaluator =
            ItemDefinitionContextEvaluator::new(&self.model_definitions)?;
        self.item_definition_type_evaluator =
            ItemDefinitionTypeEvaluator::new(&self.model_definitions)?;
        self.business_knowledge_model_evaluator =
            BusinessKnowledgeModelEvaluator::new(&self.model_definitions, self)?;
        self.decision_evaluator = DecisionEvaluator::new(&self.model_definitions, self)?;
        self.decision_service_evaluator =
            DecisionServiceEvaluator::new(&self.model_definitions, self)?;
        Ok(())
    }

    pub fn input_data_evaluator(&self) -> &InputDataEvaluator {
        &self.input_data_evaluator
    }

    pub fn input_data_context_evaluator(&self) -> &InputDataContextEvaluator {
        &self.input_data_context_evaluator
    }

    pub fn item_definition_context_evaluator(&self) -> &ItemDefinitionContextEvaluator {
        &self.item_definition_context_evaluator
    }

    pub fn item_definition_evaluator(&self) -> &ItemDefinitionEvaluator {
        &self.item_definition_evaluator
    }

    pub fn item_definition_type_evaluator(&self) -> &ItemDefinitionTypeEvaluator {
        &self.item_definition_type_evaluator
    }

    pub fn decision_evaluator(&self) -> &DecisionEvaluator {
        &self.decision_evaluator
    }

    pub fn add_decision_invocable(
        &self,
        model_namespace: String,
        model_name: String,
        invocable_name: String,
        def_key: DefKey,
    ) {
        self.invocables.borrow_mut().add_decision(
            model_namespace,
            model_name,
            invocable_name,
            def_key,
        );
    }

    pub fn add_bkm_invocable(
        &self,
        model_namespace: String,
        model_name: String,
        invocable_name: String,
        def_key: DefKey,
        output_variable_name: Name,
    ) {
        self.invocables.borrow_mut().add_bkm(
            model_namespace,
            model_name,
            invocable_name,
            def_key,
            output_variable_name,
        );
    }

    pub fn add_decision_service_invocable(
        &self,
        model_namespace: String,
        model_name: String,
        invocable_name: String,
        def_key: DefKey,
    ) {
        self.invocables.borrow_mut().add_decision_service(
            model_namespace,
            model_name,
            invocable_name,
            def_key,
        );
    }
}

impl From<ModelBuilder> for EvaluatorBuilders {
    fn from(value: ModelBuilder) -> Self {
        Self {
            input_data_evaluator: value.input_data_evaluator,
            item_definition_evaluator: value.item_definition_evaluator,
            business_knowledge_model_evaluator: value.business_knowledge_model_evaluator,
            decision_evaluator: value.decision_evaluator,
            decision_service_evaluator: value.decision_service_evaluator,
            invocables: value.invocables.into_inner(),
            information_item_types: value
                .item_definition_type_evaluator
                .information_item_types(),
        }
    }
}
