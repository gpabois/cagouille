use crate::{
    context::InitContext,
    interaction::BoundInteraction,
    interface::{Signal, Slot},
    pilot::Pilot,
    r#async::MaybeAsync,
    reaction::{AnyReaction, Reaction},
    Context,
};
use tokio::sync::{mpsc, watch};

/// The reactor's core
pub struct Core<Matter> {
    /// The matter to react
    matter: Matter,
    /// Reactions to process
    reactions: mpsc::UnboundedReceiver<AnyReaction>,
    /// shutdown button
    shutdown: watch::Receiver<bool>,
    /// Current bound interaction
    current_interaction: watch::Sender<Option<BoundInteraction>>,
    /// Signals received from outside the core
    signal: Signal,
}

impl<Matter> Core<Matter>
where
    Matter: Sync + Send + 'static,
{
    /// Create a new reactor core, and returns its pilot
    pub fn create<S>(init: MaybeAsync<InitContext<Matter>, Matter>) -> Pilot<Matter> {
        let (any_reactions_sender, any_reactions_recv) = mpsc::unbounded_channel::<AnyReaction>();
        let (shutdown_sender, shutdown_recv) = watch::channel(false);
        let (current_sender, current_recv) = watch::channel::<Option<BoundInteraction>>(None);

        // Signal to pilot the reactor.
        let signal = Signal::new(any_reactions_sender.clone());
        let slot = Slot::new(current_recv.clone());

        let reactor_signal = signal.clone();

        // The core lives within a future.
        let join = tokio::spawn(async move {
            let init_ctx = InitContext::new(reactor_signal.clone(), slot.clone());

            let core = Core {
                matter: init.call(init_ctx).await,
                reactions: any_reactions_recv,
                shutdown: shutdown_recv,
                current_interaction: current_sender,
                signal: reactor_signal.clone(),
            };

            core.r#loop().await;
        });

        Pilot::new(join, signal.clone(), shutdown_sender)
    }

    /// Run the loop
    async fn r#loop(mut self) {
        loop {
            tokio::select! {
                 Some(reaction) = self.reactions.recv() => {
                     if let Some(reaction) = reaction.downcast::<Matter>() {
                         self.process_reaction(reaction).await
                     }
                }
                 Ok(_) = self.shutdown.changed() => {
                     break;
                 }
            }
        }
    }

    /// Process the reaction
    async fn process_reaction(&mut self, reaction: Reaction<Matter>) {
        match reaction {
            Reaction::BoundInteract(bound) => {
                if let Some(interaction) = bound.downcast::<Matter>() {
                    let ctx = Context::new(&mut self.matter);
                    self.current_interaction.send(Some(bound.clone())).unwrap();
                    interaction.execute(ctx);
                    self.current_interaction.send(None).unwrap();
                    bound.ack();
                }
            }
            Reaction::Interact(interaction) => {
                let ctx = Context::new(&mut self.matter);
                let bnd = BoundInteraction::new(interaction.clone().into(), self.signal.clone());
                self.current_interaction.send(Some(bnd)).unwrap();
                interaction.execute(ctx);
                self.current_interaction.send(None).unwrap();
            }
            Reaction::Act(action) => {
                let ctx = Context::new(&mut self.matter);
                action.execute(ctx);
            }
        }
    }
}
