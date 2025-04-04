use good_lp::{Variable, VariableDefinition};

struct DiscretisedVariables {
    variables: Vec<Variable>,
}

pub fn define_variables(vars: &mut Variables, steps: usize) -> Vec<DiscretisedVariables> {
    (0..steps)
        .map(|_| {
            let a = vars.add(VariableDefinition::max(1.0));
            let b = vars.add(VariableDefinition::min(2.0).max(4.0));
            TimeStepVars { a, b }
        })
        .collect()
}
