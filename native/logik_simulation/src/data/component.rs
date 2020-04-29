#![warn(non_camel_case_types)]

/// A trait to define common behaviour between the components
pub(crate) trait Component {
    fn inputs(&self) -> usize;
    fn outputs(&self) -> usize;
    fn gor_din_grej(&self) -> Utdata;
}


/// Placeholder for now
pub(crate) struct Utdata {
    pub utdata_varde: f64,  //julius hade nån hittepå-ide om att en sladd kunde va på o av samtidigt så whatever jag bryr mig inte.
}



impl Component for Utdata {
    fn inputs(&self) -> usize {
        return 1;
    }
    
    fn outputs(&self) -> usize {
        return 0;
    }

    fn gor_din_grej(&self) -> Utdata {
        return Utdata {utdata_varde: 0.0};
    }
}

/// Placeholder for now
pub(crate) struct AND {

}

impl Component for AND {
    fn inputs(&self) -> usize {
        return 2;
    }
    
    fn outputs(&self) -> usize {
        return 1;
    }

    fn gor_din_grej(&self) -> Utdata {
        return Utdata {utdata_varde: 0.0};
    }

}

pub(crate) struct OR {

}

impl Component for OR {
    fn inputs(&self) -> usize {
        return 2;
    }
    
    fn outputs(&self) -> usize {
        return 1;
    }

    fn gor_din_grej(&self) -> Utdata {
        return Utdata {utdata_varde: 0.0};
    }
}

pub(crate) struct NOT {

}

impl Component for NOT {
    fn inputs(&self) -> usize {
        return 1;
    }
    
    fn outputs(&self) -> usize {
        return 1;
    }

    fn gor_din_grej(&self) -> Utdata {
        return Utdata {utdata_varde: 1.0};
    }
}

pub(crate) struct XOR {

}

impl Component for XOR {
    fn inputs(&self) -> usize {
        return 2;
    }
    
    fn outputs(&self) -> usize {
        return 1;
    }

    fn gor_din_grej(&self) -> Utdata {
        return Utdata {utdata_varde: 1.0};
    }
}