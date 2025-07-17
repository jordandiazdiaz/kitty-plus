#[cfg(feature = "plugins")]
use anyhow::Result;
#[cfg(feature = "plugins")]
use std::collections::HashMap;
#[cfg(feature = "plugins")]
use std::path::Path;
#[cfg(feature = "plugins")]
use wasmtime::*;

#[cfg(feature = "plugins")]
pub struct PluginManager {
    engine: Engine,
    plugins: HashMap<String, Plugin>,
}

#[cfg(feature = "plugins")]
pub struct Plugin {
    name: String,
    module: Module,
    instance: Option<Instance>,
}

#[cfg(feature = "plugins")]
impl PluginManager {
    pub fn new() -> Result<Self> {
        let engine = Engine::default();
        
        Ok(Self {
            engine,
            plugins: HashMap::new(),
        })
    }
    
    pub fn load_plugin<P: AsRef<Path>>(&mut self, name: String, path: P) -> Result<()> {
        let wasm_bytes = std::fs::read(path)?;
        let module = Module::from_binary(&self.engine, &wasm_bytes)?;
        
        let plugin = Plugin {
            name: name.clone(),
            module,
            instance: None,
        };
        
        self.plugins.insert(name, plugin);
        Ok(())
    }
    
    pub fn initialize_plugin(&mut self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(name) {
            let mut store = Store::new(&self.engine, ());
            let instance = Instance::new(&mut store, &plugin.module, &[])?;
            plugin.instance = Some(instance);
        }
        Ok(())
    }
    
    pub fn call_plugin_function(
        &mut self,
        plugin_name: &str,
        function_name: &str,
        args: &[Val],
    ) -> Result<Vec<Val>> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            if let Some(instance) = &plugin.instance {
                let mut store = Store::new(&self.engine, ());
                let func = instance
                    .get_func(&mut store, function_name)
                    .ok_or_else(|| anyhow::anyhow!("Function not found: {}", function_name))?;
                
                let mut results = vec![Val::I32(0); func.ty(&store).results().len()];
                func.call(&mut store, args, &mut results)?;
                
                return Ok(results);
            }
        }
        
        Err(anyhow::anyhow!("Plugin not found or not initialized: {}", plugin_name))
    }
    
    pub fn list_plugins(&self) -> Vec<&str> {
        self.plugins.keys().map(|s| s.as_str()).collect()
    }
    
    pub fn unload_plugin(&mut self, name: &str) -> Result<()> {
        self.plugins.remove(name);
        Ok(())
    }
}

#[cfg(not(feature = "plugins"))]
pub struct PluginManager;

#[cfg(not(feature = "plugins"))]
impl PluginManager {
    pub fn new() -> Self {
        Self
    }
}