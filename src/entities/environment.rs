const DEFAULT_ENVIRONMENT_NAME : &str = "Environment";
pub struct Environment {
    name: String,
    const_growth_rate: f64,
    growth_percentage: f64,
    current_resources: f64,
    tick: usize,
}
///
///
/// # Arguments
///
/// * `name`:
/// * `const_growth_rate`:
/// * `growth_percentage`:
/// * `starting_resources`:
///
/// returns: Environment
///
/// # Examples
///
/// ```
///
/// ```
pub fn create_environment<S>(name: Option<S>, const_growth_rate: f64, growth_percentage: f64, starting_resources: f64) -> Environment
    where
    S: Into<String>,{
    let extracted_name = name.map(|n| n.into());
    Environment {
        name: extracted_name.unwrap_or(DEFAULT_ENVIRONMENT_NAME.parse().unwrap()),
        const_growth_rate,
        growth_percentage,
        current_resources: starting_resources,
        tick: 0,
    }
}

impl Environment {
    pub fn copy(&self) -> Environment {
        Environment {
            name: self.name.clone(),
            const_growth_rate: self.const_growth_rate,
            growth_percentage: self.growth_percentage,
            current_resources : self.current_resources,
            tick: self.tick,
        }
    }
    pub fn tick(&mut self) {
        self.current_resources *= 1.0 + (self.growth_percentage / 100.0);
        self.current_resources += self.const_growth_rate;
        self.tick += 1;
    }
    
    pub fn use_resource(&mut self, target: f64) -> f64{
        if target <= 0.0 {
            0.0
        } else if target >= self.current_resources {
            let ret = self.current_resources;
            self.current_resources = 0.0;
            ret
        } else {
            self.current_resources -= target;
            target
        }
    }
    // Getters
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_const_growth_rate(&self) -> f64 {
        self.const_growth_rate
    }
    pub fn get_growth_percentage(&self) -> f64 {
        self.growth_percentage
    }
    pub fn get_current_resources(&self) -> f64 {
        self.current_resources
    }
    pub fn get_tick(&self) -> usize {
        self.tick
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_environment_creation() {
        let def = create_environment::<&str>(None, 10.0, 20.0, 10.0);
        assert_eq!(def.get_name(), DEFAULT_ENVIRONMENT_NAME);
        assert_eq!(def.get_const_growth_rate(), 10.0);
        assert_eq!(def.get_growth_percentage(), 20.0);
        assert_eq!(def.get_current_resources(), 10.0);
        assert_eq!(def.get_tick(), 0);
        let named = create_environment(Some("Environment 1"), 0.0, 50.0, 10.0);
        assert_eq!(named.get_name(), "Environment 1");
        assert_eq!(named.get_const_growth_rate(), 0.0);
        assert_eq!(named.get_growth_percentage(), 50.0);
        assert_eq!(named.get_current_resources(), 10.0);
        assert_eq!(named.get_tick(), 0);
    }

    #[test]
    fn test_environment_tick() {
        let mut def = create_environment::<&str>(None, 10.0, 20.0, 10.0);
        assert_eq!(def.get_name(), DEFAULT_ENVIRONMENT_NAME);
        assert_eq!(def.get_const_growth_rate(), 10.0);
        assert_eq!(def.get_growth_percentage(), 20.0);
        assert_eq!(def.get_current_resources(), 10.0);
        assert_eq!(def.get_tick(), 0);
        def.tick();
        assert_eq!(def.get_current_resources(), 22.0);
        assert_eq!(def.get_tick(), 1);
        def.tick();
        assert_eq!(def.get_current_resources(), 36.4);
        assert_eq!(def.get_tick(), 2);
    }

    #[test]
    fn test_environment_use_resource() {
        let mut norm = create_environment::<&str>(None, 10.0, 20.0, 10.0);
        assert_eq!(norm.get_name(), DEFAULT_ENVIRONMENT_NAME);
        assert_eq!(norm.get_const_growth_rate(), 10.0);
        assert_eq!(norm.get_growth_percentage(), 20.0);
        assert_eq!(norm.get_current_resources(), 10.0);
        assert_eq!(norm.get_tick(), 0);
        let norm_res = norm.use_resource(3.0);
        assert_eq!(norm_res, 3.0);
        assert_eq!(norm.get_name(), DEFAULT_ENVIRONMENT_NAME);
        assert_eq!(norm.get_const_growth_rate(), 10.0);
        assert_eq!(norm.get_growth_percentage(), 20.0);
        assert_eq!(norm.get_current_resources(), 7.0);
        assert_eq!(norm.get_tick(), 0);
        let second_res = norm.use_resource(-3.0);
        assert_eq!(second_res, 0.0);
        assert_eq!(norm.get_name(), DEFAULT_ENVIRONMENT_NAME);
        assert_eq!(norm.get_const_growth_rate(), 10.0);
        assert_eq!(norm.get_growth_percentage(), 20.0);
        assert_eq!(norm.get_current_resources(), 7.0);
        assert_eq!(norm.get_tick(), 0);
        let third_res = norm.use_resource(3000.0);
        assert_eq!(third_res, 7.0);
        assert_eq!(norm.get_name(), DEFAULT_ENVIRONMENT_NAME);
        assert_eq!(norm.get_const_growth_rate(), 10.0);
        assert_eq!(norm.get_growth_percentage(), 20.0);
        assert_eq!(norm.get_current_resources(), 0.0);
        assert_eq!(norm.get_tick(), 0);


    }
}