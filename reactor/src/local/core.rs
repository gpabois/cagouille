use crate::local::{Signal, SignalRx, SlotTx, Reaction, Context, BoundInteraction};

/// The reactor's core
pub struct Core<Matter> {
    matter: Matter,
    signal_rx: SignalRx,
    signal: Signal,
    slot_tx: SlotTx
}

impl<Matter> Core<Matter>
where
    Matter: 'static,
{
    pub fn new(matter: Matter, signal: Signal, signal_rx: SignalRx, slot_tx: SlotTx) -> Self {
        Self {matter, signal, signal_rx, slot_tx}
    }
    /// Run the loop
    pub async fn r#loop(mut self) {
        loop {
            if let Some(reaction) = self.signal_rx.poll_downcast::<Matter>().await {
                self.process_reaction(reaction).await
            }
        }
    }

    /// Process the reaction
    async fn process_reaction(&mut self, reaction: Reaction<Matter>) {
        match reaction {
            Reaction::BoundInteract(bound) => {
                if let Some(interaction) = bound.downcast::<Matter>() {
                    let ctx = Context::new(&mut self.matter);
                    self.slot_tx.set_current_interaction(bound.clone());
                    interaction.execute(ctx);
                    self.slot_tx.pop_current_interaction();
                    bound.ack();
                }
            }
            Reaction::Interact(interaction) => {
                let ctx = Context::new(&mut self.matter);
                let bound = BoundInteraction::new(
                    interaction.clone().into(), 
                    self.signal.clone()
                );
                self.slot_tx.set_current_interaction(bound.clone());
                interaction.execute(ctx);
                self.slot_tx.pop_current_interaction();
            }
            Reaction::Act(action) => {
                let ctx = Context::new(&mut self.matter);
                action.execute(ctx);
            }
        }
    }
}
