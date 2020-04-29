#![warn(non_camel_case_types)]

/// A trait to define common behaviour between the components
pub(crate) trait Component {
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
}


/// Placeholder for now
pub(crate) struct Output {
    pub utdata_varde: f64,  //julius hade nån hittepå-ide om att en sladd kunde va på o av samtidigt så whatever jag bryr mig inte.
}



impl Component for Output {
    fn inputs(&self) -> usize {
        1
    }
    
    fn outputs(&self) -> usize {
        0
    }
}

/// Placeholder for now
pub(crate) struct AND {

}

impl Component for AND {
    fn inputs(&self) -> usize {
        2
    }
    
    fn outputs(&self) -> usize {
        1
    }
}

pub(crate) struct OR {

}

impl Component for OR {
    fn inputs(&self) -> usize {
        2
    }
    
    fn outputs(&self) -> usize {
        1
    }
}