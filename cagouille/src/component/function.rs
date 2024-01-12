use std::{ops::DerefMut, sync::Arc};

use futures::future::BoxFuture;

use super::{State, ComponentEvent};

pub mod traits {
    use crate::futures::future::BoxFuture;
    use crate::component::traits::{self, Component};

    pub trait FunctionComponentRunner {
        type Component: traits::Component;

        fn run<'state, 'props, 'fut>(
            state: &'state mut super::FcState<Self::Component>, 
            props: &'props <Self::Component as Component>::Properties
        ) -> BoxFuture<'fut, crate::vdom::VNodeResult> where 'state: 'fut, 'props: 'fut ;
    }
}

pub struct FcState<Component> where Component: super::traits::Component {
    component_state: State<Component> 
}

impl<Component> FcState<Component> where Component: super::traits::Component {
    pub fn new(component_state: State<Component>) -> Self {
        Self{component_state}
    }
}

pub struct FunctionComponent<Component> 
where Component: super::traits::Component
{
    _never: std::marker::PhantomData<Component>,
    ctx: Arc<futures_locks::Mutex<FcState<Component>>>
}

impl<Component> FunctionComponent<Component> 
    where Component: super::traits::Component + traits::FunctionComponentRunner<Component = Component> + Sync + 'static
{
    pub fn new(component_state: State<Component>) -> Self {
        Self {
            _never: std::marker::PhantomData,
            ctx: Arc::new(futures_locks::Mutex::new(FcState::new(component_state)))
        }
    }

    pub fn render<'a>(&self, props: &'a Component::Properties) -> BoxFuture<'a, crate::vdom::VNodeResult> {
        let ctx = self.ctx.clone();
        Box::pin(async move {
            let mut ctx = ctx.lock().await;
            Component::run(ctx.deref_mut(), props).await
        })
    }

    pub async fn process_event<'state, 'fut>(&mut self, _state: &'state State<Component>, event: ComponentEvent<'_, Component>) 
    -> Result<(), crate::error::Error> {
        
        match event {
            ComponentEvent::PropertiesChanged { previous } => todo!(),
            ComponentEvent::Rendered => todo!(),
            ComponentEvent::Destroyed => todo!()
        }

        Ok(())
    }
}